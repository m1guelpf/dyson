# Setting up a Dyson instance

This is currently more of a description of what I've done than a guide on how to properly do it. I'm new to pretty much the entire stack, so take everything with a grain of salt. PRs welcome!

## 0. Hardware

The specifics of the hardware shouldn't matter (as long as you use NVIDIA GPUs), but for reference, here's what I'm using:

- GPU: 2x NVIDIA A4000
- Case: Lian-Li O11 AIR Black
- Motherboard: Asus Pro WS WRX80E-SAGE
- CPU: AMD Ryzen Threadripper PRO 5955WX
- Power: EVGA Supernova P2 1600 W 80 Plus Platinum
- Cooling: Enermax Liqtech 360 TR4 II Liquid Cooling Kit
- RAM: G.Skill Ripjaws V 128GB (4x32GB) DDR4 3600MHz
- SSD: Samsung 980 Pro SSD 1TB M.2 + Samsung 980 Pro SSD 2TB M.2

> **Warning**
> This is an incredibly overkill setup, and I wouldn't recommend copying it. Figure out what kind of models you'll want to run, find a GPU that can fit them, and build around that.

## 1. OS setup

I'm using Arch Linux, but any Linux distro should work. As with everything else in this guide, I'll note what I've done, and you should tweak it for your own setup.

Flash the latest Arch Linux ISO to a USB drive, boot into it, set up your network, and run `archinstall` (I didn't set up a graphical environment, and you probably don't need to either). Then, set up [yay](https://github.com/Jguer/yay) to make things easier.

Finally, install the NVIDIA drivers. You probably just need to `yay -S nvidia`, but check [this guide](https://wiki.archlinux.org/title/NVIDIA) just in case. Also, set up a pacman hook to automatically rebuild the NVIDIA modules on kernel updates by creating the following file at `/etc/pacman.d/hooks/nvidia.hook`:

```ini
[Trigger]
Operation=Install
Operation=Upgrade
Operation=Remove
Type=Package
Target=nvidia
Target=linux
# Change the linux part above and in the Exec line if a different kernel is used

[Action]
Description=Update NVIDIA module in initcpio
Depends=mkinitcpio
When=PostTransaction
NeedsTargets
Exec=/bin/sh -c 'while read -r trg; do case $trg in linux) exit 0; esac; done; /usr/bin/mkinitcpio -P'
```

If you did everything correctly, running `nvidia-smi` should show you the GPUs connected to your system.

## 2. Tailscale

Since this is meant to be a private server and I didn't feel like port forwarding, I'm using [Tailscale](https://tailscale.com/) to connect to it. It's good, it's free, and it just takes `yay -S tailscale` to install. Then, `systemctl start tailscaled` and `tailscale up --ssh` to connect to your account.

Note that Arch doesn't enable services by default, so you'll need to run `systemctl enable tailscaled` to make sure it starts on boot.

## 3. Docker and NVIDIA Container Toolkit

This one's easy, just run `yay -S docker docker-compose nvidia-container-toolkit`. Then, `systemctl enable docker` and `systemctl start docker` to make sure it runs and starts on boot. If you wanna avoid having to run all docker commands as root, add your user to the `docker` group with `sudo usermod -aG docker $USER`.

You can ensure everything worked by running `docker run --rm --gpus all nvidia/cuda:12.1.1-base-ubuntu22.04 nvidia-smi`, which should display the same output as `nvidia-smi` on your host.

## 4. Kubernetes

[k3s](https://k3s.io/) seems to be the simplest way get kubernetes running, so that's what I went with. Just `curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="v1.25.11+k3s1" sh -` and you're good to go (verify with `sudo kubectl get nodes`). If you want to skip sudo when running kubectl commands, run `mkdir ~/.kube && sudo cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sudo chown $USER:$USER ~/.kube/config`.

### 4.1. DNS + SSL

Even though all traffic will be routed through Tailscale, I chose to set up a domain and SSL certificates to keep things pretty. Turns out Cloudflare makes getting SSL certs for non-public servers really easy, so keep that in mind if you go with something else.

First, point your domain to your server's tailscale ip (you can get it by running `tailscale ip --4`). I'd recommend pointing both the root domain (`yourdomain.com`) and the wildcard subdomain (`*.yourdomain.com`) to make assinging subdomains easier later on.

Then, install helm with `yay -S helm` and cert-manager with `helm repo add jetstack https://charts.jetstack.io && helm repo update && helm install cert-manager jetstack/cert-manager --namespace cert-manager --create-namespace --version v1.12.0 --set installCRDs=true`.

Finally, you'll need to [create a Cloudflare API token](https://dash.cloudflare.com/profile/api-tokens) with `Zone:Zone:Read` and `Zone:DNS:Edit` permissions, and run `CF_TOKEN="your-cloudflare-token" make cf-secret` and `CF_EMAIL="your-cloudflare-email" make cf-issuer`.

### 4.2. Rancher

While you don't need Rancher to run Dyson, I've found it makes k8s a little less scary by letting you click around instead of remembering a bunch of commands or writing tons of YAML. Here's how to set it up:

```bash
# Add the rancher repo to helm
helm repo add rancher-latest https://releases.rancher.com/server-charts/latest
# Pull data from the repo we just added
helm repo update
# Install rancher (make sure to replace YOUR_ADMIN_PASSWORD with a password of your choice and rancher.yourdomain.com with your actual (sub)domain)
helm install rancher rancher-latest/rancher --namespace cattle-system --create-namespace --set hostname=rancher.yourdomain.com --set ingress.tls.source=secret --version v2.7.5 --set replicas=1 --set bootstrapPassword=YOUR_ADMIN_PASSWORD
```

Finally, generate the certificate with `RANCHER_DOMAIN="rancher.yourdomain.com" make rancher-cert`.

It usually takes around a minute to start up (you can watch progress with `kubectl -n cattle-system rollout status deploy/rancher`). Once it's done, you should now be able to access Rancher at the (sub)domain you configured from any machine in your tailscale network and log in with the password you set above.

## CI/CD

In order to simplify the process of deploying Dyson updates on push, this repo uses a self-hosted Github Actions runner. You can probably skip this step unless you plan on forking and tweaking Dyson.

To set up the runner, go to your repo's Settings page, and then click on "New self-hosted runner" under Actions > Runners and follow the instructions provided by GitHub, except for running `sudo ./svc.sh install && sudo ./svc.sh start` instead of `./run.sh`. This will install the runner as a service, so it'll start on boot.

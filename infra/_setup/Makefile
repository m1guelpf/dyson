cf-secret:
	@kubectl delete -f cloudflare/cf-secret.yaml --ignore-not-found --grace-period=0
	@sed -e "s;{{CF_TOKEN}};$(CF_TOKEN);g" cloudflare/cf-secret.yaml | kubectl create -f -
cf-issuer:
	@kubectl delete -f cloudflare/cf-issuer.yaml --ignore-not-found --grace-period=0
	@sed -e "s;{{CF_EMAIL}};$(CF_EMAIL);g" cloudflare/cf-issuer.yaml | kubectl create -f -
rancher-cert:
	@kubectl delete -f rancher/rancher-cert.yaml --ignore-not-found --grace-period=0
	@sed -e "s;{{RANCHER_DOMAIN}};$(RANCHER_DOMAIN);g" rancher/rancher-cert.yaml | kubectl create -f -

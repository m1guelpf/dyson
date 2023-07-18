local_path_storage = local('curl https://raw.githubusercontent.com/rancher/local-path-provisioner/v0.0.24/deploy/local-path-storage.yaml')
k8s_yaml(local_path_storage)

k8s_yaml('infra/namespace.yaml')
k8s_yaml('infra/db.yaml')
include('./api/Tiltfile')
include('./frontend/Tiltfile')

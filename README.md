# Kubernetes Custom Controller for Rust

## Sample Custom Controller

Build container image.

```sh
buildah bud --format=docker -t <Custom Controller Image Path> -f sample-custom-controller/Dockerfile .
```

Push container image.

```sh
podman push <Custom Controller Image Path>
```

Deploy custom resource definition.

```sh
crdgen | kubectl apply -f -
```

Create namespace.

```sh
kubectl create namespace sample-system
```

Deploy cutom controller to target namespace.

```sh
cat | kubectl apply -f - <<EOF
apiVersion: v1
kind: ServiceAccount
metadata:
  name: sample-custom-controller
  namespace: sample-system
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: sample-custom-controller
rules:
- apiGroups:
  - sample.custom-controller
  resources:
  - samples
  - samples/status
  verbs:
  - get
  - list
  - patch
  - watch
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: sample-custom-controller
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: sample-custom-controller
subjects:
- kind: ServiceAccount
  name: sample-custom-controller
  namespace: sample-system
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: sample-custom-controller
  namespace: sample-system
spec:
  selector:
    matchLabels:
      app: sample-custom-controller
  template:
    metadata:
      labels:
        app: sample-custom-controller
    spec:
      serviceAccountName: sample-custom-controller
      containers:
      - name: sample-custom-controller
        image: <Custom Controller Image Path>
        env:
        - name: RUST_LOG
          value: info
EOF
```

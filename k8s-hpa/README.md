# Setup

## Install a Cluster

```bash
curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC="--disable traefik --prefer-bundled-bin" sh -s -
```

### Make kubectl work with k3s cluster

```bash
sudo cp /etc/rancher/k3s/k3s.yaml ~/.kube/config && sudo chown $USER ~/.kube/config && sudo chmod 600 ~/.kube/config && export KUBECONFIG=~/.kube/config
```

### Remove default `metrics-server` from cluster

```bash
kubectl delete -n kube-system service metrics-server
kubectl delete -n kube-system deployment metrics-server
```

### Setup Prometheus Helm Repo (if not already set up)

```bash
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update
```

### Create Monitoring namespace

```bash
kubectl create namespace monitoring
```

This namespace will contain all of the resources related to the prometheus monitoring stack, including the operator as well as the adapter.

## Install Prometheus Operator

```bash
kubectl create -f prometheus-operator/crds.yaml
kubectl create -f prometheus-operator/operator.yaml
kubectl apply -f prometheus.yaml
```

This creates the `CustomResourceDefinitions` for the prometheus-operator. It enables the use of special resources in the cluster like the `Prometheus` and `ServiceMonitor` resource. It also installs the operator and launches an instance of Prometheus (`prometheus-operated` / `prometheus-prometheus`) which scrapes metrics from any given `ServiceMonitor` and `PodMonitor`.

## Deploy Sample App

```bash
kubectl apply -f sample-app/
```

This is an example application that exposes a `http_requests_total` metric under the `/metrics` path on its `web` port (80).
It also creates a `ServiceMonitor` which tells the operated prometheus to watch this `web` port.

You should now be able to see that the `http_requests_total` metric is being scraped by looking at the Prometheus UI.

## Install Prometheus Adapter

```bash
helm install prometheus-adapter -n monitoring prometheus-community/prometheus-adapter
helm upgrade -i prometheus-adapter -n monitoring prometheus-community/prometheus-adapter -f prometheus-adapter/values.yaml
```

This deploys the prometheus-adapter and patches it so it finds the operated prometheus instance and exposes a custom k8s metric called `my_custom_metric` based on the rate of the `http_requests_total` prometheus metric we are collecting.

## Configure HPA for Sample App

```bash
kubectl apply -f sample-app.hpa.yaml
```

This creates a `HorizontalPodAutoscaler` resource for the sample-app, which scales the amount of pods from 1 to 10 based on the `my_custom_metric` metric with a target average value (averaged over all pods for this deployment) of 500 milli-requests per second.

## Generate Load on the sample-app and watch as new pods are created by the HPA

Run this command a bunch of times:

```bash
curl http://$(kubectl get service sample-app -o jsonpath='{ .spec.clusterIP }')/metrics
```

After a while you should see that there are now more sample-app pods automatically created to handle the high load. These will also automatically scale back once the load drops again and matches or falls below the target average value.

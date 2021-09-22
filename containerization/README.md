### Container Image Building

Run the following from the root folder of the repo:

- sudo docker build -t container-registry-url/paws-server:v1.0.0 .

- sudo docker push container-registry-url/paws-server:v1.0.0

***container-registry-url*** is where container image will be hosted.

### <u>Kubernetes Setup</u>

k0s was used, the distro is not so much important, you can use other solutions like kind, microk8s, k3s etc

- curl -sSLf https://get.k0s.sh | sudo sh
- sudo k0s install controller --single
- sudo k0s start
- sudo k0s status
- sudo k0s kubeconfig admin > .kube/config    (This is to get the kubeconfig so that kubectl can be used normally, alternative is to use the "sudo k0s kubectl")

```
chris@paws:~/paws$ kubectl get no -o wide
NAME   STATUS   ROLES    AGE   VERSION       INTERNAL-IP      EXTERNAL-IP   OS-IMAGE             KERNEL-VERSION   
paws   Ready    <none>   26h   v1.22.1+k0s   192.168.178.41   <none>        Ubuntu 20.04.1 LTS   5.4.0-59-generic

```

##### Install Nginx Ingress controller

```
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update

helm install ingress-nginx ingress-nginx/ingress-nginx -n ingress-nginx \
--set controller.admissionWebhooks.enabled=false \
--set controller.service.type=NodePort --set controller.service.nodePorts.http=32080 \
--set controller.service.nodePorts.https=32443 \
--create-namespace

kubectl apply -f paws/containerization/nginx-ingress/ingress-class.yaml

```

##### Create Image Pull Token And TLS Certificate Secrets

```
kubectl apply -f paws/containerization/secrets
```

### <u>PAWs Application Installation</u>

- **cd paws/containerization/helm-chart**
- **helm install paws .**

```
chris@paws:~/paws/containerization/helm-chart$ helm install paws .
NAME: paws
LAST DEPLOYED: Tue Sep 21 12:31:16 2021
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
NOTES:
Status after deployment:

$(kubectl --namespace default get all)
chris@paws:~/paws/containerization/helm-chart$ 
chris@paws:~/paws/containerization/helm-chart$ kubectl get po
NAME                           READY   STATUS    RESTARTS   AGE
paws-server-6c7bf6dcb6-2jgzp   1/1     Running   0          8s
chris@paws:~/paws/containerization/helm-chart$ 
chris@paws:~/paws/containerization/helm-chart$ 
chris@paws:~/paws/containerization/helm-chart$ 
chris@paws:~/paws/containerization/helm-chart$ kubectl get ing
NAME          CLASS   HOSTS             ADDRESS   PORTS     AGE
paws-server   nginx   paws-server.org             80, 443   12s
```

Test API call via the VM IP address and nodePort:

```
chris@paws:~/paws/containerization/helm-chart$ curl -s -k --resolve paws-server.org:32443:192.168.178.41 https://paws-server.org:32443/v1beta/paws/version | jq
{
  "pawsVersion": "1.0"
}



curl -s -k --resolve paws-server.org:32443:192.168.178.41 https://paws-server.org:32443/v1beta/paws/init \
--header 'Content-Type: application/json' \
--data-raw '{
"jsonrpc": "2.0",
"method": "spectrum.paws.init",
"params": {
    "type": "INIT_REQ",
    "version": "1.0",
    "deviceDesc": {
      "serialNumber": "XXX",
      "fccId": "YYY",
      "rulesetIds": ["NccTvBandWhiteSpace-2019"]
     },
    "location": {
        "point": {
            "center": {"latitude": 37.0, "longitude": -101.3}
        }
    }
},
"id": "idString"
}' | jq


{
  "jsonrpc": "2.0",
  "result": {
    "type": "INIT_RESP",
    "version": "1.0",
    "rulesetInfos": [
      {
        "authority": "ng",
        "rulesetId": "NccTvBandWhiteSpace-2019",
        "maxLocationChange": 100,
        "maxPollingSecs": 86400
      }
    ]
  },
  "id": "xxxxxx"
}
```


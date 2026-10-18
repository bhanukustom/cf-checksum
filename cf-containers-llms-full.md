---
description: Run serverless containers alongside Workers to handle resource-intensive workloads, custom runtimes, and existing container images on Cloudflare.
title: Containers
image: https://developers.cloudflare.com/containers/og.png?v=9068707d88e89d36
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Containers

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Enhance your Workers with serverless containers

Available on Workers Paid plan

Run code written in any programming language, built for any runtime, as part of apps built on [Workers](https://developers.cloudflare.com/workers).

Deploy your container image to `Region:Earth` without worrying about managing infrastructure - just define your Worker and [`wrangler deploy`](https://developers.cloudflare.com/workers/wrangler/commands/general/#deploy).

With Containers you can run:

- Resource-intensive applications that require CPU cores running in parallel, large amounts of memory or disk space
- Applications and libraries that require a full filesystem, specific runtime, or Linux-like environment
- Existing applications and tools that have been distributed as container images

Container instances are spun up on-demand and controlled by code you write in your [Worker](https://developers.cloudflare.com/workers). Instead of chaining together API calls or writing Kubernetes operators, you just write TypeScript or JavaScript.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";
import { rewriteRequestForContainer, startAndWaitForPort } from "./utils";

export class MyContainer extends DurableObject {
	async fetch(request) {
		const container = this.ctx.container;
		await startAndWaitForPort(container, {
			port: 4000,
			inactivityTimeout: 10 * 60 * 1000,
		});

		const forwarded = rewriteRequestForContainer(request);
		const response = await container.getTcpPort(4000).fetch(forwarded);
		return response;
	}
}

export default {
	async fetch(request, env) {
		const containerRequest = request.clone();
		const { "session-id": sessionId } = await request.json();
		return env.MY_CONTAINER.getByName(sessionId).fetch(containerRequest);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";
import { rewriteRequestForContainer, startAndWaitForPort } from "./utils";

interface Env {
	MY_CONTAINER: DurableObjectNamespace<MyContainer>;
}

export class MyContainer extends DurableObject<Env> {
	async fetch(request: Request): Promise<Response> {
		const container = this.ctx.container!;
		await startAndWaitForPort(container, {
			port: 4000,
			inactivityTimeout: 10 * 60 * 1000,
		});

		const forwarded = rewriteRequestForContainer(request);
		const response = await container.getTcpPort(4000).fetch(forwarded);
		return response;
	}
}

export default {
	async fetch(request: Request, env: Env): Promise<Response> {
		const containerRequest = request.clone();
		const { "session-id": sessionId } = await request.json<{
			"session-id": string;
		}>();
		return env.MY_CONTAINER.getByName(sessionId).fetch(containerRequest);
	},
};
```

*src/utils.jsjs*

```js
export function rewriteRequestForContainer(request) {
	const url = new URL(request.url);
	url.protocol = "http:";
	url.host = "container";
	const forwarded = new Request(url, request);
	forwarded.headers.delete("host");
	return forwarded;
}

export async function startAndWaitForPort(container, options) {
	await container.setInactivityTimeout(options.inactivityTimeout);
	if (!container.running) {
		container.start();
	}

	// start() returns before the container is ready, so poll its health endpoint.
	const port = container.getTcpPort(options.port);
	let lastError;
	for (let attempt = 0; attempt < 100; attempt++) {
		try {
			const response = await port.fetch("http://container/health");
			if (!response.ok) {
				throw new Error(`Health check returned ${response.status}`);
			}
			return;
		} catch (error) {
			lastError = error;
			await scheduler.wait(200);
		}
	}
	throw new Error(`Container did not become ready on port ${options.port}`, {
		cause: lastError,
	});
}
```

*src/utils.tsts*

```ts
interface ReadinessOptions {
	port: number;
	inactivityTimeout: number;
}

type ContainerInstance = NonNullable<DurableObjectState["container"]>;

export function rewriteRequestForContainer(request: Request): Request {
	const url = new URL(request.url);
	url.protocol = "http:";
	url.host = "container";
	const forwarded = new Request(url, request);
	forwarded.headers.delete("host");
	return forwarded;
}

export async function startAndWaitForPort(
	container: ContainerInstance,
	options: ReadinessOptions,
): Promise<void> {
	await container.setInactivityTimeout(options.inactivityTimeout);
	if (!container.running) {
		container.start();
	}

	// start() returns before the container is ready, so poll its health endpoint.
	const port = container.getTcpPort(options.port);
	let lastError: unknown;
	for (let attempt = 0; attempt < 100; attempt++) {
		try {
			const response = await port.fetch("http://container/health");
			if (!response.ok) {
				throw new Error(`Health check returned ${response.status}`);
			}
			return;
		} catch (error) {
			lastError = error;
			await scheduler.wait(200);
		}
	}
	throw new Error(`Container did not become ready on port ${options.port}`, {
		cause: lastError,
	});
}
```

```jsonc
{
	"name": "container-starter",
	"main": "src/index.ts",
	"upload_source_maps": true,
	// Set this to today's date
	"compatibility_date": "2026-10-14",
	"containers": [
		{
			"class_name": "MyContainer",
			"image": "./Dockerfile",
			"max_instances": 5,
		},
	],
	"durable_objects": {
		"bindings": [
			{
				"class_name": "MyContainer",
				"name": "MY_CONTAINER",
			},
		],
	},
	"exports": {
		"MyContainer": {
			"type": "durable-object",
			"storage": "sqlite",
		},
	},
}
```

```toml
name = "container-starter"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[[containers]]
class_name = "MyContainer"
image = "./Dockerfile"
max_instances = 5

[[durable_objects.bindings]]
class_name = "MyContainer"
name = "MY_CONTAINER"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"
```

[Get started](https://developers.cloudflare.com/containers/get-started/) [Containers dashboard](https://dash.cloudflare.com/?to=/:account/workers/containers)

---

## Next steps

[Get started](https://developers.cloudflare.com/containers/get-started/)

Build and push an image, call a Container from a Worker, and try scaling and routing.

Deploy a Container

[Examples](https://developers.cloudflare.com/containers/examples/)

Stateless and stateful routing, regional placement, Workflow and Queue integrations, AI-generated code execution, and short-lived workloads.

See Examples

[Local development](https://developers.cloudflare.com/containers/guides/local-dev/)

Run your Worker and container together with `wrangler dev` or `vite dev` before you deploy.

Develop locally

[Deploy](https://developers.cloudflare.com/containers/guides/deploy/)

Ship from your machine or Workers Builds, and confirm the deploy.

Deploy Containers

---

## More resources

### [Scheduling policies](https://developers.cloudflare.com/containers/configuration/scheduling-policy/)

Choose whether image and instance configuration is managed centrally or from Durable Object code.

### [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/)

How container instances update after you deploy.

### [Image management](https://developers.cloudflare.com/containers/guides/image-management/)

Build, push, and pull images for Containers.

### [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/)

How a container is scheduled, started, routed, and shut down.

### [Limits](https://developers.cloudflare.com/containers/platform/limits/)

Instance counts, image size, and other platform limits.

### [Wrangler](https://developers.cloudflare.com/workers/wrangler/commands/containers/#containers)

CLI commands for images and containers.

### [APIs](https://developers.cloudflare.com/containers/api/)

Use the Durable Object Container API or reference the Container class.

### [SSH](https://developers.cloudflare.com/containers/guides/ssh/)

Connect to running container instances with SSH through Wrangler.

### [Containers Discord](https://discord.cloudflare.com)

Ask questions, show what you are building, and talk with other Containers developers.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/#page","headline":"Containers","description":"Run serverless containers alongside Workers to handle resource-intensive workloads, custom runtimes, and existing container images on Cloudflare.","url":"https://developers.cloudflare.com/containers/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/og.png?v=9068707d88e89d36","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Deploy your first Container on Cloudflare by building an image, configuring a Worker, and routing requests to container instances.
title: Get started
image: https://developers.cloudflare.com/containers/get-started/og.png?v=82557fc11168d06b
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Get started

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/get-started/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

In this guide, you will deploy a Worker that can make requests to one or more Containers in response to end-user requests. In this example, each container runs a small webserver written in Go.

This example Worker should give you a sense for simple Container use, and provide a starting point for more complex use cases.

This guide uses the `Container` class and its built-in lifecycle helpers. You can also manage containers directly through `ctx.container`. To compare both options, refer to [Containers APIs](https://developers.cloudflare.com/containers/api/).

## Prerequisites

### Ensure Docker is running locally

In this guide, we will build and push a container image alongside your Worker code. By default, this process uses [Docker ↗︎](https://www.docker.com/) to do so.

You must have Docker running locally when you run `wrangler deploy`. For most people, the best way to install Docker is to follow the [docs for installing Docker Desktop ↗︎](https://docs.docker.com/desktop/). Other tools like [Colima ↗︎](https://github.com/abiosoft/colima) may also work.

You can check that Docker is running properly by running the `docker info` command in your terminal. If Docker is running, the command will succeed. If Docker is not running, the `docker info` command will hang or return an error including the message "Cannot connect to the Docker daemon".

## Deploy your first Container

Run the following command to create and deploy a new Worker with a container, from the starter template:

npmyarnpnpm

```
npm create cloudflare@latest -- --template=cloudflare/templates/containers-template
```

```
yarn create cloudflare --template=cloudflare/templates/containers-template
```

```
pnpm create cloudflare@latest --template=cloudflare/templates/containers-template
```

When you want to deploy a code change to either the Worker or Container code, you can run the following command using [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/):

npmyarnpnpm

```
npx wrangler deploy
```

```
yarn wrangler deploy
```

```
pnpm wrangler deploy
```

On deploy, Wrangler uploads your Worker, builds and pushes the container image with Docker, and updates container instances on Cloudflare's network. The first build and push usually take the longest. Later deploys [reuse cached image layers ↗︎](https://docs.docker.com/build/cache/).

Note

After you deploy your Worker for the first time, wait several minutes before you expect container requests to succeed. The Worker URL may respond while Cloudflare is still provisioning containers. During that time, calls into the container can error.

### Check deployment status

After deploying, list containers in your account and their status:

npmyarnpnpm

```
npx wrangler containers list
```

```
yarn wrangler containers list
```

```
pnpm wrangler containers list
```

List images in the Cloudflare Registry:

npmyarnpnpm

```
npx wrangler containers images list
```

```
yarn wrangler containers images list
```

```
pnpm wrangler containers images list
```

### Make requests to Containers

Open the URL for your Worker. It should look like `https://hello-containers.<YOUR_WORKERS_SUBDOMAIN>.workers.dev`.

- Requests to `/container/1` or `/container/2` route to specific containers. Each path after `/container/` maps to a unique container.
- Requests to `/lb` load-balance across three containers chosen at random.

Read the response body to confirm which instance handled the request. If the Worker responds but container routes still error, wait for provisioning, then check [Containers ↗︎](https://dash.cloudflare.com/?to=/:account/workers/containers) logs in the dashboard.

## Understanding the Code

Now that you've deployed your first container, let's explain what is happening in your Worker's code, in your configuration file, in your container's code, and how requests are routed.

### Configuration

Your [Wrangler configuration file](https://developers.cloudflare.com/containers/configuration/wrangler/) defines the configuration for both your Worker and your container:

```jsonc
{
	"containers": [
		{
			"max_instances": 10,
			"class_name": "MyContainer",
			"image": "./Dockerfile",
		},
	],
	"durable_objects": {
		"bindings": [
			{
				"name": "MY_CONTAINER",
				"class_name": "MyContainer",
			},
		],
	},
	"exports": {
		"MyContainer": {
			"type": "durable-object",
			"storage": "sqlite",
		},
	},
}
```

```toml
[[containers]]
max_instances = 10
class_name = "MyContainer"
image = "./Dockerfile"

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"
```

Important points about this config:

- `image` points to a Dockerfile, to a directory containing a Dockerfile, or to a fully qualified image reference such as `registry.cloudflare.com/<YOUR_ACCOUNT_ID>/<IMAGE>:<TAG>`.
- `class_name` must be a [Durable Object class name](https://developers.cloudflare.com/durable-objects/api/base/).
- `max_instances` declares the maximum number of simultaneously running container instances that will run.
- The `exports` entry declares the Durable Object class with SQLite storage. Existing applications that use the legacy `migrations` array can continue to use it, but `exports` and `migrations` cannot be combined.

### The Container Image

Your container image must be able to run on the `linux/amd64` architecture, but aside from that, has few limitations.

In the example you just deployed, it is a simple Golang server that responds to requests on port 8080 using the `MESSAGE` environment variable that will be set in the Worker and an [auto-generated environment variable](https://developers.cloudflare.com/containers/configuration/environment-variables/) `CLOUDFLARE_DEPLOYMENT_ID.`

```go
func handler(w http.ResponseWriter, r *http.Request) {
	message := os.Getenv("MESSAGE")
	instanceId := os.Getenv("CLOUDFLARE_DEPLOYMENT_ID")

	fmt.Fprintf(w, "Hi, I'm a container and this is my message: %s, and my instance ID is: %s", message, instanceId)
}
```

Note

After deploying the example code, to deploy a different image, you can replace the provided image with one of your own.

### Worker code

#### Container Configuration

First note `MyContainer` which extends the [`Container` ↗︎](https://github.com/cloudflare/containers) class:

```js
export class MyContainer extends Container {
  defaultPort = 8080;
  sleepAfter = '10s';
  envVars = {
    MESSAGE: 'I was passed in via the container class!',
  };

  override onStart() {
    console.log('Container successfully started');
  }

  override onStop() {
    console.log('Container successfully shut down');
  }

  override onError(error: unknown) {
    console.log('Container error:', error);
  }
}
```

This defines basic configuration for the container:

- `defaultPort` sets the port that the `fetch` and `containerFetch` methods will use to communicate with the container. It also blocks requests until the container is listening on this port.
- `sleepAfter` sets the timeout for the container to sleep after it has been idle for a certain amount of time.
- `envVars` sets environment variables that will be passed to the container when it starts.
- `onStart`, `onStop`, and `onError` are hooks that run when the container starts, stops, or errors, respectively.

The `Container` class itself extends [`DurableObject`](https://developers.cloudflare.com/durable-objects/), so your subclass has access to the full Durable Object API. The Durable Object handles routing, lifecycle, and persistent state, while the container process runs your image inside a Linux VM. This means you can use [`this.ctx.storage`](https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/) to persist data that survives container restarts and resides close to the container itself.

For all properties and methods, refer to the [Container class API](https://developers.cloudflare.com/containers/api/container-class/) and the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/).

#### Routing to Containers

When a request enters Cloudflare, your Worker's [`fetch` handler](https://developers.cloudflare.com/workers/runtime-apis/handlers/fetch/) is invoked. This is the code that handles the incoming request. The fetch handler in the example code, launches containers in two ways, on different routes:

- Making requests to `/container/` passes requests to a new container for each path. This is done by spinning up a new Container instance. You may note that the first request to a new path takes longer than subsequent requests, this is because a new container is booting.

  ```js
  if (pathname.startsWith("/container")) {
  	const container = env.MY_CONTAINER.getByName(pathname);
  	return await container.fetch(request);
  }
  ```

- Making requests to `/lb` will load balance requests across several containers. This uses a simple `getRandom` helper method, which picks an ID at random from a set number (in this case 3), then routes to that Container instance. You can replace this with any routing or load balancing logic you choose to implement:

  ```js
  if (pathname.startsWith("/lb")) {
  	const container = await getRandom(env.MY_CONTAINER, 3);
  	return await container.fetch(request);
  }
  ```

This allows for multiple ways of using Containers:

- If you simply want to send requests to many stateless and interchangeable containers, you should load balance.
- If you have stateful services or need individually addressable containers, you should request specific Container instances.
- If you are running short-lived jobs, want fine-grained control over the container lifecycle, want to parameterize container entrypoint or env vars, or want to chain together multiple container calls, you should request specific Container instances.

Note

Today, routing requests to one of many interchangeable Container instances uses the `getRandom` helper.

It randomly selects one of a fixed number of instances for each request.

## View Containers in your Dashboard

The [Containers Dashboard ↗︎](https://dash.cloudflare.com/?to=/:account/workers/containers) shows you helpful information about your Containers, including:

- Status and Health
- Metrics
- Logs

After launching your Worker, go to the Containers Dashboard by selecting **Workers & Pages** > **Containers** in the dashboard sidebar.

## Next Steps

To do more:

- Modify the image by changing the Dockerfile and running `wrangler deploy`
- Refer to [Deploy Containers](https://developers.cloudflare.com/containers/guides/deploy/) for Workers Builds and rollout behavior
- Browse [examples](https://developers.cloudflare.com/containers/examples/) for more patterns
- Check the [Frequently Asked Questions](https://developers.cloudflare.com/containers/faq/) for platform behavior and limitations

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/get-started/#page","headline":"Get started","description":"Deploy your first Container on Cloudflare by building an image, configuring a Worker, and routing requests to container instances.","url":"https://developers.cloudflare.com/containers/get-started/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/get-started/og.png?v=82557fc11168d06b","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Understand what a Cloudflare Container is, how requests reach it, how it starts and stops, where it runs, and how it holds state.
title: Container Fundamentals
image: https://developers.cloudflare.com/containers/concepts/og.png?v=23cdeac259988f35
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Container Fundamentals

Last updated Sep 28, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/concepts/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

A Cloudflare Container runs your container image in a full Linux environment on Cloudflare's network, alongside a Worker. The Worker handles incoming HTTP requests, while the Container provides the runtime, binaries, and languages your application needs. Together, they let you run Linux workloads without managing the underlying infrastructure.

## The Container runtime

*Run Linux workloads alongside your Worker.*

A Worker runs code in an isolated JavaScript environment. A Container runs your image in a full Linux environment, so you can bring custom runtimes and binaries when your application needs them. Each Container instance runs inside its own virtual machine, which isolates it from other workloads. The Worker receives inbound HTTP requests and routes them to the Container, so the Container is not exposed directly to end users.

Play Reset

Request

WORKER

For more information, refer to [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/).

## How requests reach a Container

*Route every request through your Worker.*

The request begins at your Worker, which uses the [Durable Object](https://developers.cloudflare.com/durable-objects/) layer to identify and reach a Container. This layer coordinates routing and lifecycle behavior, allowing the Container to continue serving requests while it runs. A Container can start when it receives its first request, so the first request may take longer to complete.

Choose an instance identity to send a request

AB Reset

Request

WORKERRequest

WORKER

DURABLE OBJECTAROUTEHOOKSSLEEP

DURABLE OBJECTBROUTEHOOKSSLEEP

DURABLE OBJECTROUTEHOOKSSLEEP

DURABLE OBJECTROUTEHOOKSSLEEP

For more information, refer to [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/).

## Container lifecycle

*Start on demand, then stop when idle.*

A Container starts when your application needs it, so the first request can take longer while its environment starts. Once running, it serves requests until it becomes inactive; after an inactivity period, it can stop and release its running resources. When another request arrives, the Container can start again.

Play Reset

Request

Awaiting requestnothing runs untila request arrives

For more information, refer to [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/) and [Pricing](https://developers.cloudflare.com/containers/platform/pricing/).

## Sizing

*Choose resources for your workload.*

Container instance sizes let you match resources to your workload. You can choose a predefined size or configure a custom one, with larger sizes providing more CPU, memory, and disk. When choosing a size, consider the resources your application needs and refer to [Limits and instance types](https://developers.cloudflare.com/containers/platform/limits/) for the available options and constraints.

For more information, refer to [Pricing](https://developers.cloudflare.com/containers/platform/pricing/).

## Placement

*Place instances near users or within constraints.*

Cloudflare places Container instances across its network, helping applications serve users from suitable locations. You can constrain placement by region or jurisdiction when your workload has location or data residency requirements. You can also run multiple instances when your application needs more capacity.

Choose a location  
 to send a request

Reset

Location A

Location B

Location C

Container

Container

Container

SchedulerSend requestSend requestSend request

For more information, refer to [Placement](https://developers.cloudflare.com/containers/concepts/placement/).

## Storage and connectivity

*Keep durable data outside the local disk.*

Treat a Container's local disk as temporary working space, because data on it can be lost when the Container stops or restarts. For data that must persist, use Durable Object storage or Cloudflare storage bindings such as KV, R2, and D1. Configured outbound handlers let the Container access these bindings and external services without requiring an SDK inside the Container.

Reset Play

Client

Worker

Container instancerunningimagemy-app:latestlocal disk

Persistent storage

KV

R2

D1

Durable Objects

Idle — click Next to step through the request

For more information, refer to [Connect to Workers and bindings](https://developers.cloudflare.com/containers/configuration/workers-connections/) and [Outbound traffic](https://developers.cloudflare.com/containers/configuration/outbound-traffic/).

## Start building

When you are ready to build, start with a deployed Container or explore the available guides.

### [Get started](https://developers.cloudflare.com/containers/get-started/)

Deploy your first container.

### [Guides](https://developers.cloudflare.com/containers/guides/)

Task-focused walkthroughs for common container workloads.

## Related resources

- [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/): the deep dive on deployment, routing, and shutdown.
- [Placement](https://developers.cloudflare.com/containers/concepts/placement/): where instances run and how to constrain them.
- [Connect to Workers and bindings](https://developers.cloudflare.com/containers/configuration/workers-connections/): reaching Cloudflare resources from a container.
- [Limits and instance types](https://developers.cloudflare.com/containers/platform/limits/) and [Pricing](https://developers.cloudflare.com/containers/platform/pricing/): sizes, account limits, and billing.
- [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/): how new versions roll out across instances.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/concepts/#page","headline":"Container Fundamentals","description":"Understand what a Cloudflare Container is, how requests reach it, how it starts and stops, where it runs, and how it holds state.","url":"https://developers.cloudflare.com/containers/concepts/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/concepts/og.png?v=23cdeac259988f35","dateModified":"2026-09-28","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Understand how a Container is deployed, started, routed, and shut down across Cloudflare's network.
title: Lifecycle of a Container
image: https://developers.cloudflare.com/containers/concepts/architecture/og.png?v=9fb00333f140e961
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Lifecycle of a Container

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/concepts/architecture/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

## Deployment

How images and running Container instances update depends on the [scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/) for the application.

With the `durable_object` policy, Wrangler prepares the named images for the application. Durable Object code can access their immutable references and selects an image and instance size when it calls `ctx.container.start()`. When an image reference changes, the Durable Object code decides whether to stop the running Container and start it with the new image or let it continue with the previous image. This application-controlled restart is how you roll out image changes with the `durable_object` policy. These instances do not participate in application-wide image rollouts.

With the `default` policy, Wrangler uploads or resolves the application image. Cloudflare distributes that image across its network and prepares capacity for new instances. Changes to the image or instance type use a [rollout](https://developers.cloudflare.com/containers/configuration/rollouts/).

Worker code goes live on deploy before any application-wide Container rollout finishes. Refer to [Deploy Containers](https://developers.cloudflare.com/containers/guides/deploy/).

## Container instance lifecycle

```
flowchart LR
    accTitle: Container instance lifecycle
    accDescr: A Worker accesses a container through its Durable Object. The container moves from stopped to starting, running but not ready, ready for traffic, stopping, and stopped.

    Worker["Worker"] -->|Durable Object binding| DurableObject["Durable Object"]
    DurableObject -->|<code>ctx.container</code>| Container

    subgraph Container["Container instance"]
        direction TD
        Stopped["Stopped"]
        Starting["Starting<br/><code>start()</code> called"]
        Running["Running<br/>Not ready"]
        Ready["Ready<br/>Accepting traffic"]
        Stopping["Stopping<br/>Stop requested"]
        StoppedAgain["Stopped"]

        Stopped --> Starting --> Running --> Ready --> Stopping --> StoppedAgain
    end

```

A Container can only be accessed through its [Durable Object](https://developers.cloudflare.com/durable-objects/). A Worker sends a request to the Durable Object, which accesses the Container through `ctx.container`.

An inactivity timeout, `signal()`, `destroy()`, a rollout, or a process exit can stop the instance. If startup fails or the process exits early, the instance returns to the stopped state.

The `ctx.container.running` property becomes `true` before the process is ready to accept traffic. Check port readiness before you send the first request.

For new applications, manage this lifecycle through the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/). It lets the Durable Object coordinate container compute with persistent state and alarms. Existing applications may use the [`Container` class](https://developers.cloudflare.com/containers/api/container-class/). To compare both APIs, refer to [Containers APIs](https://developers.cloudflare.com/containers/api/).

## Lifecycle of a request

### Client to Worker

Recall that Containers are backed by [Durable Objects](https://developers.cloudflare.com/durable-objects/) and [Workers](https://developers.cloudflare.com/workers/). Requests are first routed through a Worker, which is generally handled by a datacenter in a location with the best latency between itself and the requesting user. A different datacenter may be selected to optimize overall latency, if [Smart Placement](https://developers.cloudflare.com/workers/configuration/placement/) is on, or if the nearest location is under heavy load.

Because all Container requests are passed through a Worker, end-users cannot make non-HTTP TCP or UDP requests to a Container instance. If you have a use case that requires inbound TCP or UDP from an end-user, please [let us know ↗︎](https://forms.gle/AGSq54VvUje6kmKu8).

### Worker to Durable Object

From the Worker, a request passes through a Durable Object instance. You can extend `DurableObject` and use `ctx.container` directly, or extend the [`Container` class](https://developers.cloudflare.com/containers/api/container-class/). Each Durable Object instance is a globally routable isolate that can execute code and store state. This allows developers to address and route to specific container instances, run code when a container exits, and store persistent state associated with each instance.

### Starting a Container

When a Durable Object requests a new Container instance, Cloudflare selects eligible capacity with the required image available. The `durable_object` policy uses the `image` and `instance` options supplied to `ctx.container.start()`. The `default` policy uses the application image and instance type from Wrangler configuration.

Note

Durable Objects and their associated Container instances are not guaranteed to run in the same location.

Container placement is optimized for request routing and startup speed, so a Container may start in a different location than its Durable Object.

Starting additional Container instances can use other locations where the image is available. Cloudflare prepares additional capacity as demand grows. Because prepared capacity is finite, some Container instances may start in locations farther from the end user. You are only charged for actively running instances, not for prepared images that are not running.

#### Cold starts

A cold start is when a container instance is started from a completely stopped state. If you call `env.MY_CONTAINER.get(id)` with a completely novel ID and launch this instance for the first time, it will result in a cold start. This will start the container image from its entrypoint for the first time. Depending on what this entrypoint does, it will take a variable amount of time to start.

Container cold starts can often be in the 1-3 second range, but this is dependent on image size and code execution time, among other factors.

### Requests to running Containers

When a request *starts* a new container instance, the nearest location with a pre-fetched image is selected. Subsequent requests to a particular instance, regardless of where they originate, will be routed to this location as long as the instance stays alive.

However, once that container instance stops and restarts, future requests could be routed to a *different* location. This location will again be the nearest location to the originating request with a pre-fetched image.

### Container runtime

Each container instance runs in a Firecracker microVM with its own kernel and network. No other workload on the Cloudflare network shares that kernel. Your image runs as a Linux container inside the VM.

Containers should be built for the `linux/amd64` architecture, and should stay within [size limits](https://developers.cloudflare.com/containers/platform/limits/).

[Logging](https://developers.cloudflare.com/containers/faq/#how-do-container-logs-work), metrics collection, and [networking](https://developers.cloudflare.com/containers/faq/#how-do-i-allow-or-disallow-egress-from-my-container) are automatically set up on each container, as configured by the developer.

### Container shutdown

With the Durable Object Container API, call [`setInactivityTimeout()`](https://developers.cloudflare.com/containers/api/durable-object-container/#setinactivitytimeout) to let the runtime stop the container after the Durable Object becomes inactive. The Durable Object becomes inactive when it stops receiving requests. The timeout can keep the container available while the Durable Object sleeps. You can also stop a container with [`signal()`](https://developers.cloudflare.com/containers/api/durable-object-container/#signal) or [`destroy()`](https://developers.cloudflare.com/containers/api/durable-object-container/#destroy).

The `Container` class sets [`sleepAfter`](https://developers.cloudflare.com/containers/api/container-class/#sleepafter) to 10 minutes by default. Its [`onActivityExpired()`](https://developers.cloudflare.com/containers/api/container-class/#onactivityexpired) implementation calls [`stop()`](https://developers.cloudflare.com/containers/api/container-class/#stop). You can change the duration or override the hook.

When the platform is about to stop a container instance, it:

1. Sends `SIGTERM` to the main process in the container.
2. Waits up to 15 minutes for that process to exit.
3. Sends `SIGKILL` if the process is still running.

Handle `SIGTERM` in your image if you need cleanup before exit. The same sequence runs when a [rollout](https://developers.cloudflare.com/containers/configuration/rollouts/) replaces a container instance with a new image.

### Lifecycle events

The Durable Object Container API provides [`monitor()`](https://developers.cloudflare.com/containers/api/durable-object-container/#monitor). Its promise resolves when the container exits and rejects when the container errors.

The [`Container` class](https://developers.cloudflare.com/containers/api/container-class/) adds hooks that run Worker code when the container changes state:

- [`onStart()`](https://developers.cloudflare.com/containers/api/container-class/#onstart) — Runs after the container has started.
- [`onStop()`](https://developers.cloudflare.com/containers/api/container-class/#onstop) — Runs after the container process exits. Receives the exit code and reason for the stop.
- [`onActivityExpired()`](https://developers.cloudflare.com/containers/api/container-class/#onactivityexpired) — Runs when the [`sleepAfter`](https://developers.cloudflare.com/containers/api/container-class/#sleepafter) timer expires with no incoming requests. The default implementation calls `stop()` to shut down the container. You can use this to only stop the container on certain conditions.
- [`onError()`](https://developers.cloudflare.com/containers/api/container-class/#onerror) — Runs when container startup or port checking fails.

Refer to the [status hooks example](https://developers.cloudflare.com/containers/examples/status-hooks/) for a full implementation.

#### Use snapshots

All disk is ephemeral by default. When a Container instance goes to sleep, the next time it starts, it uses a fresh disk from the container image.

If you need point-in-time filesystem state, Container applications that use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy) can create and restore a snapshot. Snapshots are immutable, so later file changes require a new snapshot. For more information, refer to [Snapshots](https://developers.cloudflare.com/containers/guides/snapshots/).

You can also use [FUSE](https://developers.cloudflare.com/containers/examples/r2-fuse-mount/) to persist disk to R2 or other object storage backends. Though you should not expect native SSD-like performance while using FUSE.

## An example request

- A developer deploys a Container. Cloudflare automatically readies instances across its Network.
- A request is made from a client in Bariloche, Argentina. It reaches the Worker in a nearby Cloudflare location in Neuquen, Argentina.
- This Worker request calls `getContainer(env.MY_CONTAINER, "session-1337")`. Under the hood, this brings up a Durable Object, which then calls `this.ctx.container.start`.
- This requests the nearest free Container instance. Cloudflare recognizes that an instance is free in Buenos Aires, Argentina, and starts it there.
- A different user needs to route to the same container. This user's request reaches the Worker running in Cloudflare's location in San Diego, US.
- The Worker again calls `getContainer(env.MY_CONTAINER, "session-1337")`.
- If the initial container instance is still running, the request is routed to the original location in Buenos Aires. If the initial container has gone to sleep, Cloudflare will once again try to find the nearest "free" instance of the Container, likely one in North America, and start an instance there.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/concepts/architecture/#page","headline":"Lifecycle of a Container","description":"Understand how a Container is deployed, started, routed, and shut down across Cloudflare's network.","url":"https://developers.cloudflare.com/containers/concepts/architecture/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/concepts/architecture/og.png?v=9fb00333f140e961","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Control where your containers run with regional and jurisdictional constraints.
title: Placement
image: https://developers.cloudflare.com/containers/concepts/placement/og.png?v=4dec4faf359f60ac
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Placement

Last updated Oct 1, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/concepts/placement/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

By default, containers run in the location nearest to the incoming request with a pre-fetched image. Use placement constraints to restrict where your containers run for data residency, compliance, or latency requirements.

## Regional constraints

Use the `regions` constraint to limit container placement to specific geographic areas:

| Region | Description | Notes |
| --- | --- | --- |
| `ENAM` | Eastern North America | |
| `WNAM` | Western North America | |
| `EEUR` | Eastern Europe | |
| `WEUR` | Western Europe | |
| `APAC` | Asia Pacific | |
| `SAM` | South America | |
| `ME` | Middle East | Limited capacity |
| `OC` | Oceania | Limited capacity |
| `AFR` | Africa | Limited capacity |

Limited capacity regions (ME, OC, AFR) cannot be used exclusively. Include at least one other region, or contact support for dedicated access.

## Jurisdictional constraints

Use the `jurisdiction` constraint to restrict containers to compliance boundaries:

| Jurisdiction | Regions | Use case |
| --- | --- | --- |
| `eu` | EEUR, WEUR | EU data residency |
| `us` | ENAM, WNAM | US data residency |
| `fedramp` | ENAM, WNAM | FedRAMP regions |

When you specify both `jurisdiction` and `regions`, the regions must be valid for that jurisdiction. For example, specifying `jurisdiction: "eu"` with `regions: ["ENAM"]` is invalid.

## Configure placement

Set placement constraints in your Wrangler configuration:

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "containers": [
    {
      "name": "my-container",
      "image": "docker.io/my-org/my-image:latest",
      "constraints": {
        "regions": [
          "ENAM",
          "WNAM"
        ],
        "jurisdiction": "fedramp"
      }
    }
  ]
}
```

```toml
[[containers]]
name = "my-container"
image = "docker.io/my-org/my-image:latest"

[containers.constraints]
regions = ["ENAM", "WNAM"]
jurisdiction = "fedramp"
```

Refer to [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/) for more details on how placement affects container startup and routing.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/concepts/placement/#page","headline":"Placement","description":"Control where your containers run with regional and jurisdictional constraints.","url":"https://developers.cloudflare.com/containers/concepts/placement/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/concepts/placement/og.png?v=4dec4faf359f60ac","dateModified":"2026-10-01","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Task-focused guides for deploying containers, developing locally, managing images, executing commands, and connecting over SSH.
title: Guides
image: https://developers.cloudflare.com/containers/guides/og.png?v=8bb82aa98ac15f19
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Guides

Last updated Aug 28, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/guides/#page","headline":"Guides","description":"Task-focused guides for deploying containers, developing locally, managing images, executing commands, and connecting over SSH.","url":"https://developers.cloudflare.com/containers/guides/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/og.png?v=8bb82aa98ac15f19","dateModified":"2026-08-28","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Deploy from your machine or Workers Builds, including how images and container instances update.
title: Deploy Containers
image: https://developers.cloudflare.com/containers/guides/deploy/og.png?v=e911f81b77c0f8dc
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Deploy Containers

Last updated Sep 22, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/deploy/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

## Deploy from your machine

1. If `image` in your Wrangler config is a path to a Dockerfile, start [Docker ↗︎](https://www.docker.com/) or another Docker-compatible engine. Specify the Dockerfile itself, not its directory. If `image` is a registry reference (Cloudflare Registry, Docker Hub, Amazon ECR, or Google Artifact Registry), you do not need Docker at deploy time. Refer to [Image management](https://developers.cloudflare.com/containers/guides/image-management/).
2. From your project directory, run:npmyarnpnpm

   ```
   npx wrangler deploy
   ```

   ```
   yarn wrangler deploy
   ```

   ```
   pnpm wrangler deploy
   ```

3. Wait for the command to finish.

`wrangler deploy` uploads and activates your Worker before it processes the container configuration. For a Dockerfile image, Wrangler then builds and pushes the image when needed. For a registry image, it uses the configured image reference. These steps are not transactional: an image build, image push, or [rollout](https://developers.cloudflare.com/containers/configuration/rollouts/) error can happen after the new Worker is already live.

For an existing container application, Wrangler starts a rollout when the effective container configuration changes. The command does not wait for every container instance to be replaced, so new Worker code may briefly talk to containers that still run the previous image. The first deploy creates the container application directly, and a deploy with no effective container changes starts no rollout.

First deploy

The first deploy can take several minutes while Cloudflare provisions the image. The Worker URL may respond before container routes succeed.

For rollout flags and step configuration, refer to [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/).

## Deploy with Workers Builds

[Workers Builds](https://developers.cloudflare.com/workers/ci-cd/builds/) runs the build and deploy commands you configure for the Worker that is connected to your Git repository.

| Git branch | Default deploy command | Containers |
| --- | --- | --- |
| Production branch | `npx wrangler deploy` | Publishes the image when needed and rolls out container instances. Dockerfile builds can run in the Workers Builds environment. |
| Other branches (if [non-production branch builds](https://developers.cloudflare.com/workers/ci-cd/builds/build-branches/) are enabled) | `npx wrangler versions upload` | Uploads Worker code only. Does not publish a new image or roll out container instances. |

### Production

1. In the Cloudflare dashboard, go to **Workers & Pages**, open the Worker you want to deploy, then go to **Settings** > **Builds**. [Go to **Workers & Pages** ↗](https://dash.cloudflare.com/?to=/:account/workers-and-pages)
2. Set **Deploy command** to `npx wrangler deploy`, or to a package script that runs a full deploy.
3. Keep your Wrangler config and Dockerfile or image reference under the Workers Builds [root directory](https://developers.cloudflare.com/workers/ci-cd/builds/configuration/).
4. Push to your [production branch](https://developers.cloudflare.com/workers/ci-cd/builds/build-branches/).
5. Confirm the build succeeds, then [check the deploy](#check-your-deploy).

To connect a repository, refer to [Workers Builds](https://developers.cloudflare.com/workers/ci-cd/builds/).

### Before production

- **`wrangler deploy`** publishes container images (when needed) and rolls out container instances for the Worker you deploy.
- **`wrangler versions upload`** (the default non-production branch deploy command in Workers Builds) uploads a new Worker version only. It does not publish a new image or roll out container instances.
- **[Version URLs](https://developers.cloudflare.com/workers/versions-and-deployments/version-urls/) are not generated for Workers that implement [Durable Objects](https://developers.cloudflare.com/durable-objects/)**, which includes Containers Workers. A successful non-production Workers Builds run still creates a Worker version, but not a full-app Version URL.

| Goal | What to do |
| --- | --- |
| Change Worker and container together on your machine | [Local development](https://developers.cloudflare.com/containers/guides/local-dev/) with `wrangler dev` |
| Share a deployed environment with its own image and container instances | A [Wrangler environment](https://developers.cloudflare.com/workers/wrangler/environments/) or a separate Worker, each connected to Workers Builds with a full deploy command such as `npx wrangler deploy --env staging`. Refer to [Workers Builds and Wrangler environments](https://developers.cloudflare.com/workers/ci-cd/builds/advanced-setups/#wrangler-environments). |
| Update production | Merge to the production branch (or run `wrangler deploy` locally) |

Caution

Do not set the Workers Builds **non-production branch deploy command** on your production Worker to `wrangler deploy` only to try a new image from a feature branch. That command deploys the same Worker that already serves production traffic, so it can roll the container instances users already hit. Use local development, or connect a separate staging Worker or Wrangler environment to Workers Builds instead.

## Check your deploy

After `wrangler deploy` or a production Workers Builds deploy:

1. Confirm the new Worker deployment is active in the dashboard.
2. Send a request that must reach the container and confirm the behavior you expect.
3. Optionally run `npx wrangler containers list` and `npx wrangler containers images list`, or open the Containers dashboard: [Go to **Containers** ↗](https://dash.cloudflare.com/?to=/:account/workers/containers)

For gradual steps, grace periods, and rollout modes, refer to [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/).

## Troubleshooting

| Problem | What to do |
| --- | --- |
| Non-production Workers Builds run succeeds but you cannot fully preview the app | Use [local development](https://developers.cloudflare.com/containers/guides/local-dev/) or a staging Worker or environment with `wrangler deploy`. Refer to [Before production](#before-production). |
| Deploy or Workers Builds run succeeds but container instances look unchanged | A gradual rollout may still be running, or the command did not update containers (`versions upload` or `--containers-rollout=none`). Refer to [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/). |
| Deploy fails because Docker is missing | Required only when `image` is a Dockerfile path. Start Docker, use Workers Builds, switch to a [registry image](https://developers.cloudflare.com/containers/guides/image-management/#use-pre-built-container-images), or use `--containers-rollout=none` for a Worker-only deploy. |
| First deploy: Worker works, container routes error | Wait several minutes for provisioning, then check logs. |

## Related

- [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/)
- [Image management](https://developers.cloudflare.com/containers/guides/image-management/)
- [Local development](https://developers.cloudflare.com/containers/guides/local-dev/)
- [Workers Builds](https://developers.cloudflare.com/workers/ci-cd/builds/)
- [Wrangler environments](https://developers.cloudflare.com/workers/wrangler/environments/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/deploy/#page","headline":"Deploy Containers","description":"Deploy from your machine or Workers Builds, including how images and container instances update.","url":"https://developers.cloudflare.com/containers/guides/deploy/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/deploy/og.png?v=e911f81b77c0f8dc","dateModified":"2026-09-22","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Run additional processes inside an active Container.
title: Execute commands
image: https://developers.cloudflare.com/containers/guides/execute-commands/og.png?v=1357ed940a06bbcf
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Execute commands

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/execute-commands/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Use `exec()` to start another process inside a running [Container](https://developers.cloudflare.com/containers/api/container-class/). The examples call `this.ctx.container.exec()` inside a class extending `Container` from `@cloudflare/containers`.

`exec()` does not start a stopped Container. In remote procedure call (RPC) methods, check `this.ctx.container.running` and call `await this.start()` when needed. You can also use the `onStart()` hook to run any series of commands whenever the Container starts.

## Run a process after startup

The following hook runs a preparation command whenever the Container starts. You can execute any series of startup commands from this hook. `output()` buffers standard output and standard error as separate `ArrayBuffer` values.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async onStart() {
		const process = await this.ctx.container.exec([
			"node",
			"scripts/prepare.js",
		]);
		const output = await process.output();
		const decoder = new TextDecoder();

		if (output.exitCode !== 0) {
			throw new Error(
				`Container preparation failed: ${decoder.decode(output.stderr)}`,
			);
		}

		console.log(decoder.decode(output.stdout));
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	override async onStart() {
		const process = await this.ctx.container.exec([
			"node",
			"scripts/prepare.js",
		]);
		const output = await process.output();
		const decoder = new TextDecoder();

	if (output.exitCode !== 0) {
		throw new Error(
			`Container preparation failed: ${decoder.decode(output.stderr)}`,
		);
	}

	console.log(decoder.decode(output.stdout));
    }

}
```

In an RPC method, ensure the Container is running before calling `exec()`. Standard output uses a readable stream by default.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async readVersion() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["node", "--version"]);
		const stdout = process.stdout
			? await new Response(process.stdout).text()
			: "";
		const exitCode = await process.exitCode;

		return { pid: process.pid, stdout, exitCode };
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async readVersion() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["node", "--version"]);
		const stdout = process.stdout
			? await new Response(process.stdout).text()
			: "";
		const exitCode = await process.exitCode;

		return { pid: process.pid, stdout, exitCode };
	}
}
```

The returned `pid` identifies the new process. The `exitCode` promise resolves when that process exits.

## Pass arguments directly

The `exec()` operation starts the executable directly with the provided argument array. It does not start a shell first.

Each array item becomes one argument. Shell features such as pipes, redirects, globbing, and variable expansion do not run implicitly.

Invoke a shell when your command needs those features. Use `["bash", "-lc", "<COMMAND>"]` if Bash exists in your image. Use `["sh", "-c", "<COMMAND>"]` if the image only provides a Portable Operating System Interface (POSIX) shell. Pass untrusted values as separate arguments instead of interpolating them into a shell command string.

## Send standard input

Pass a `ReadableStream` to send existing data. Setting `stdout` to `"ignore"` discards standard output.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async importData(data) {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const stdin = new ReadableStream({
			start(controller) {
				controller.enqueue(new TextEncoder().encode(data));
				controller.close();
			},
		});
		const process = await this.ctx.container.exec(["cat"], {
			stdin,
			stdout: "ignore",
		});
		const output = await process.output();

		return {
			stdoutBytes: output.stdout.byteLength,
			exitCode: output.exitCode,
		};
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async importData(data: string) {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const stdin = new ReadableStream<Uint8Array>({
			start(controller) {
				controller.enqueue(new TextEncoder().encode(data));
				controller.close();
			},
		});
		const process = await this.ctx.container.exec(["cat"], {
			stdin,
			stdout: "ignore",
		});
		const output = await process.output();

		return {
			stdoutBytes: output.stdout.byteLength,
			exitCode: output.exitCode,
		};
	}
}
```

Ignored standard output produces an empty buffer from `output()`. Set `stdin` to `"pipe"` to write data over time.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async concatenateInput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["cat"], {
			stdin: "pipe",
		});
		const writer = process.stdin?.getWriter();

		if (!writer) {
			throw new Error("Standard input is unavailable");
		}

		const encoder = new TextEncoder();
		await writer.write(encoder.encode("first\n"));
		await writer.write(encoder.encode("second\n"));
		await writer.close();

		const output = await process.output();
		return new TextDecoder().decode(output.stdout);
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async concatenateInput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["cat"], {
			stdin: "pipe",
		});
		const writer = process.stdin?.getWriter();

		if (!writer) {
			throw new Error("Standard input is unavailable");
		}

		const encoder = new TextEncoder();
		await writer.write(encoder.encode("first\n"));
		await writer.write(encoder.encode("second\n"));
		await writer.close();

		const output = await process.output();
		return new TextDecoder().decode(output.stdout);
	}
}
```

Close the writer to send end-of-file (EOF). If you omit `stdin`, `exec()` closes standard input and sends EOF immediately.

### Pass an RPC stream to standard input

RPC methods can accept byte-oriented `ReadableStream` values whose underlying source uses `type: "bytes"`. A `Request` body meets this requirement. You can pass the received stream directly to `exec()` without buffering the entire stream in the Durable Object. For more information, refer to [Streams over RPC](https://developers.cloudflare.com/workers/runtime-apis/rpc/#readablestream-writablestream-request-and-response).

```js
import { Container, getContainer } from "@cloudflare/containers";

export class MyContainer extends Container {
	async writeFile(input) {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["tee", "/tmp/upload.bin"], {
			stdin: input,
			stdout: "ignore",
		});

		return process.exitCode;
	}
}

export default {
	async fetch(request, env) {
		if (!request.body) {
			return new Response("Request body required", { status: 400 });
		}

		const container = getContainer(env.MY_CONTAINER, "upload-worker");
		const exitCode = await container.writeFile(request.body);

		return Response.json({ exitCode });
	},
};
```

```ts
import { Container, getContainer } from "@cloudflare/containers";

export class MyContainer extends Container {
	async writeFile(input: ReadableStream<Uint8Array>) {
		if (!this.ctx.container.running) {
			await this.start();
		}

	const process = await this.ctx.container.exec(
		["tee", "/tmp/upload.bin"],
		{
			stdin: input,
			stdout: "ignore",
		},
	);

	return process.exitCode;
    }

}

export default {
	async fetch(request: Request, env: Env): Promise<Response> {
		if (!request.body) {
			return new Response("Request body required", { status: 400 });
		}

	const container = getContainer(env.MY_CONTAINER, "upload-worker");
	const exitCode = await container.writeFile(request.body);

	return Response.json({ exitCode });
    },

};
```

RPC transfers ownership of the stream to the Durable Object. The calling Worker cannot read it after passing it to `writeFile()`.

The following `cat` process exits because standard input is omitted:

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async verifyEndOfFile() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["cat"]);
		const output = await process.output();

		return {
			stdoutBytes: output.stdout.byteLength,
			exitCode: output.exitCode,
		};
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async verifyEndOfFile() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["cat"]);
		const output = await process.output();

		return {
			stdoutBytes: output.stdout.byteLength,
			exitCode: output.exitCode,
		};
	}
}
```

## Set the process context

Use `cwd`, `env`, and `user` to set the process context. The process receives only the variables in `env`, plus `PATH`. It does not inherit other variables from `envVars` or the image. `PATH` comes from `envVars`, or from the image when `envVars` does not set it. To give a command a value from `envVars`, pass it in `env` as well.

This example uses `sh` because it needs expansion and redirection. It also captures standard output and standard error separately.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async inspectWorkspace() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(
			[
				"sh",
				"-c",
				'printf "%s:%s:%s" "$PWD" "$MODE" "$EXTRA_VALUE"; printf "diagnostic" >&2',
			],
			{
				cwd: "/workspace",
				env: {
					MODE: "inspection",
					EXTRA_VALUE: "added",
				},
			},
		);
		const output = await process.output();
		const decoder = new TextDecoder();

		return {
			stdout: decoder.decode(output.stdout),
			stderr: decoder.decode(output.stderr),
		};
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async inspectWorkspace() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(
			[
				"sh",
				"-c",
				'printf "%s:%s:%s" "$PWD" "$MODE" "$EXTRA_VALUE"; printf "diagnostic" >&2',
			],
			{
				cwd: "/workspace",
				env: {
					MODE: "inspection",
					EXTRA_VALUE: "added",
				},
			},
		);
		const output = await process.output();
		const decoder = new TextDecoder();

		return {
			stdout: decoder.decode(output.stdout),
			stderr: decoder.decode(output.stderr),
		};
	}
}
```

The `user` option sets the numeric Linux user and group IDs for the process, as `uid:gid`. When omitted, the process runs as the image user. Before you rely on `user` to restrict a process, refer to [`exec()`](https://developers.cloudflare.com/containers/api/durable-object-container/#exec).

## Combine standard error

Set `stderr` to `"combined"` to merge standard error into standard output. Combined output requires `stdout: "pipe"`.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async readCombinedOutput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(
			[
				"bash",
				"-lc",
				'printf "standard output\n"; printf "standard error\n" >&2',
			],
			{
				stdout: "pipe",
				stderr: "combined",
			},
		);
		const output = await process.output();

		return new TextDecoder().decode(output.stdout);
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async readCombinedOutput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(
			[
				"bash",
				"-lc",
				'printf "standard output\n"; printf "standard error\n" >&2',
			],
			{
				stdout: "pipe",
				stderr: "combined",
			},
		);
		const output = await process.output();

		return new TextDecoder().decode(output.stdout);
	}
}
```

The merged stream does not guarantee ordering between source streams. In this mode, `process.stderr` is `undefined`, and `output.stderr` is an empty `ArrayBuffer`. This example assumes Bash exists in the image.

## Handle nonzero exits

A nonzero exit code resolves `exitCode` normally. It does not reject the promise.

This example preserves standard error while ignoring standard output:

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async runCheck() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(
			[
				"sh",
				"-c",
				'printf "not captured"; printf "check failed\n" >&2; exit 7',
			],
			{ stdout: "ignore" },
		);
		const output = await process.output();

		return {
			exitCode: output.exitCode,
			stdoutBytes: output.stdout.byteLength,
			stderr: new TextDecoder().decode(output.stderr),
		};
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async runCheck() {
		if (!this.ctx.container.running) {
			await this.start();
		}

	const process = await this.ctx.container.exec(
		[
			"sh",
			"-c",
			'printf "not captured"; printf "check failed\n" >&2; exit 7',
		],
		{ stdout: "ignore" },
	);
	const output = await process.output();

	return {
		exitCode: output.exitCode,
		stdoutBytes: output.stdout.byteLength,
		stderr: new TextDecoder().decode(output.stderr),
	};
    }

}
```

The result contains exit code `7` and the standard error text. Its ignored standard output buffer has zero bytes.

## Stream large output

`output()` buffers both streams in memory. For large output, drain `stdout` and `stderr` concurrently instead.

```js
import { Container } from "@cloudflare/containers";

async function countBytes(stream) {
	if (!stream) {
		return 0;
	}

	let bytes = 0;
	for await (const chunk of stream) {
		bytes += chunk.byteLength;
	}
	return bytes;
}

export class MyContainer extends Container {
	async generateLargeOutput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec([
			"sh",
			"-c",
			'i=0; while [ "$i" -lt 100000 ]; do printf "output %s\n" "$i"; printf "error %s\n" "$i" >&2; i=$((i + 1)); done',
		]);

		const [stdoutBytes, stderrBytes, exitCode] = await Promise.all([
			countBytes(process.stdout),
			countBytes(process.stderr),
			process.exitCode,
		]);

		return { stdoutBytes, stderrBytes, exitCode };
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

async function countBytes(stream: ReadableStream<Uint8Array> | null) {
	if (!stream) {
		return 0;
	}

	let bytes = 0;
	for await (const chunk of stream) {
		bytes += chunk.byteLength;
	}
	return bytes;
}

export class MyContainer extends Container {
	async generateLargeOutput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec([
			"sh",
			"-c",
			'i=0; while [ "$i" -lt 100000 ]; do printf "output %s\n" "$i"; printf "error %s\n" "$i" >&2; i=$((i + 1)); done',
		]);

		const [stdoutBytes, stderrBytes, exitCode] = await Promise.all([
			countBytes(process.stdout),
			countBytes(process.stderr),
			process.exitCode,
		]);

		return { stdoutBytes, stderrBytes, exitCode };
	}
}
```

Streaming and `output()` are alternative consumption methods. `output()` throws a `TypeError` if either stream has started being consumed. A second call to `output()` also throws a `TypeError`.

### Return standard output over RPC

Return a `ReadableStream` from an RPC method to stream output to the calling Worker. Combining standard error provides one stream for both output channels.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async streamCommandOutput() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(
			["sh", "-c", 'printf "starting\n"; run-report'],
			{ stderr: "combined" },
		);

		return process.stdout;
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async streamCommandOutput(): Promise<ReadableStream<Uint8Array>> {
		if (!this.ctx.container.running) {
			await this.start();
		}

	const process = await this.ctx.container.exec(
		["sh", "-c", 'printf "starting\n"; run-report'],
		{ stderr: "combined" },
	);

	return process.stdout!;
    }

}
```

RPC transfers ownership of the stream to the caller and preserves flow control. The caller must consume or cancel the stream. If the caller stops reading, backpressure can pause a process that continues writing.

This method transfers output, not the `ExecProcess` handle. Define a separate application protocol when the caller needs completion metadata or process control.

## Stop a process

`exec()` has no built-in timeout. You can request termination after a delay with `kill()` and then await `exitCode`.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async runWithTimeout() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["sleep", "120"]);
		const timer = setTimeout(() => process.kill(), 30_000);

		try {
			return await process.exitCode;
		} finally {
			clearTimeout(timer);
		}
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async runWithTimeout() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const process = await this.ctx.container.exec(["sleep", "120"]);
		const timer = setTimeout(() => process.kill(), 30_000);

		try {
			return await process.exitCode;
		} finally {
			clearTimeout(timer);
		}
	}
}
```

Calling `kill()` without an argument queues a `SIGTERM`, signal `15`. You can pass another signal when the process requires it. A process can handle or ignore a signal, so this is not a hard execution deadline. Observe completion through `exitCode`, and do not infer a specific exit code from a signal.

### Stop the processes a command starts

`kill()` signals only the process that `exec()` started. Processes that this process starts, such as the commands in a `bash -c` script, keep running after `exitCode` resolves. `output()` waits until they exit.

To stop a command together with the processes it starts, run it under GNU coreutils `timeout`. The image must include `timeout`.

```js
const process = await this.ctx.container.exec([
	"timeout",
	"--kill-after=5",
	"30",
	"bash",
	"-c",
	"npm test",
]);
const output = await process.output();

// 124 when the command times out, or 137 if timeout also sends SIGKILL.
console.log(output.exitCode);
```

```ts
const process = await this.ctx.container.exec([
	"timeout",
	"--kill-after=5",
	"30",
	"bash",
	"-c",
	"npm test",
]);
const output = await process.output();

// 124 when the command times out, or 137 if timeout also sends SIGKILL.
console.log(output.exitCode);
```

After 30 seconds, `timeout` sends `SIGTERM` to its process group. The group includes every process the command starts, unless a process moves to its own group, for example with `setsid`. With `--kill-after=5`, `timeout` sends `SIGKILL` to the group five seconds later if the command is still running.

## Coordinate operations

Place `exec()` calls in the Durable Object that controls the Container. The Durable Object can coordinate process state and Container lifecycle.

One application RPC method can perform multiple `exec()` operations. Each command remains a separate exec operation, but the caller makes one Durable Object RPC call. This reduces caller-to-Durable Object round trips while keeping lifecycle decisions together.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async runDiagnostics() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const commands = [
			["uname", "-a"],
			["node", "--version"],
		];
		const decoder = new TextDecoder();
		const results = [];

		for (const command of commands) {
			const process = await this.ctx.container.exec(command);
			const output = await process.output();
			results.push({
				command,
				exitCode: output.exitCode,
				stdout: decoder.decode(output.stdout),
				stderr: decoder.decode(output.stderr),
			});
		}

		return results;
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async runDiagnostics() {
		if (!this.ctx.container.running) {
			await this.start();
		}

		const commands = [
			["uname", "-a"],
			["node", "--version"],
		];
		const decoder = new TextDecoder();
		const results = [];

		for (const command of commands) {
			const process = await this.ctx.container.exec(command);
			const output = await process.output();
			results.push({
				command,
				exitCode: output.exitCode,
				stdout: decoder.decode(output.stdout),
				stderr: decoder.decode(output.stderr),
			});
		}

		return results;
	}
}
```

For all fields and return types, refer to the [`exec()` API contract](https://developers.cloudflare.com/containers/api/durable-object-container/#exec).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/execute-commands/#page","headline":"Execute commands","description":"Run additional processes inside an active Container.","url":"https://developers.cloudflare.com/containers/guides/execute-commands/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/execute-commands/og.png?v=1357ed940a06bbcf","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Configure and update Container images with the durable_object and default scheduling policies.
title: Image Management
image: https://developers.cloudflare.com/containers/guides/image-management/og.png?v=3fd9a8c2d3fb3253
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Image Management

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/image-management/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Container applications manage images differently based on their [scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/). With the `durable_object` policy, Durable Object code selects an image each time it starts a Container. With the `default` policy, Wrangler configuration sets one image for the application.

## Use images with the `durable_object` scheduling policy

### Use the Cloudflare-managed image

The `cloudflare/debian-trixie` managed image is available only with the `durable_object` scheduling policy:

| Managed image | Source image | Contents |
| --- | --- | --- |
| `cloudflare/debian-trixie` | [`node:24.20.0-trixie-slim` ↗︎](https://hub.docker.com/layers/library/node/24.20.0-trixie-slim/images/sha256-a747ad80c8a161b650d79a6da9c422005b91148b18b8d2c669eb5a0b7c07e600) pinned by digest | Node.js 24.20.0 on Debian Trixie slim |

`cloudflare/debian-trixie` is a Cloudflare-managed identifier, not a public Cloudflare Docker Hub image. Pass the identifier when the Durable Object starts its Container:

*src/index.jsjs*

```js
this.ctx.container.start({
	image: "cloudflare/debian-trixie",
	enableInternet: false,
});
```

*src/index.tsts*

```ts
this.ctx.container.start({
	image: "cloudflare/debian-trixie",
	enableInternet: false,
});
```

### Configure named images

Configure one or more named images in the `images` map. Each entry must use exactly one image source. Set `dockerfile` to build an image from a local Dockerfile. Set `image` to use a digest-pinned image from the Cloudflare managed registry.

The following configuration defines one image from each source:

```jsonc
{
	"containers": [
		{
			"class_name": "AgentComputer",
			"scheduling_policy": "durable_object",
			"images": {
				"base": {
					"dockerfile": "./container/Dockerfile",
					"build_context": ".",
				},
				"tools": {
					"image": "registry.cloudflare.com/<ACCOUNT_ID>/<REPOSITORY>@sha256:<DIGEST>",
				},
			},
		},
	],
}
```

```toml
[[containers]]
class_name = "AgentComputer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./container/Dockerfile"
build_context = "."

[containers.images.tools]
image = "registry.cloudflare.com/<ACCOUNT_ID>/<REPOSITORY>@sha256:<DIGEST>"
```

Wrangler builds or resolves each named image and exposes its digest-pinned reference through `ctx.container.images`. Select a reference when the Durable Object starts its Container:

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class AgentComputer extends DurableObject {
	startContainer() {
		this.ctx.container.start({
			image: this.ctx.container.images.base,
			instance: "standard-2",
			enableInternet: false,
		});
	}
}
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

export class AgentComputer extends DurableObject {
	startContainer() {
		this.ctx.container.start({
			image: this.ctx.container.images.base,
			instance: "standard-2",
			enableInternet: false,
		});
	}
}
```

A configuration can contain up to 100 named images. Each image name must contain between 1 and 128 characters. For the complete policy configuration, refer to [Scheduling Policies](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy).

### Roll out a named image update

To update a named image, change its Dockerfile or set `image` to a new digest-pinned reference. Then, deploy the Worker. After the deployment completes, the corresponding value in `ctx.container.images` refers to the updated image. During a [Workers gradual deployment](https://developers.cloudflare.com/workers/versions-and-deployments/gradual-deployments/), each Durable Object sees the image map of the Worker version that runs it, so different Durable Objects can start different images until the deployment completes.

Deploying an updated image map does not restart running Containers. A running Container continues to use its startup image. Durable Object code decides when to stop that Container and start it with the updated reference. This application-controlled restart is how you roll out image changes with the `durable_object` policy.

To keep a running Container on its current image, use the configured image only on its next start:

*src/index.jsjs*

```js
if (!this.ctx.container.running) {
	this.ctx.container.start({
		image: this.ctx.container.images.base,
		instance: "standard-2",
		enableInternet: false,
	});
}
```

*src/index.tsts*

```ts
if (!this.ctx.container.running) {
	this.ctx.container.start({
		image: this.ctx.container.images.base,
		instance: "standard-2",
		enableInternet: false,
	});
}
```

To upgrade a running Container immediately, compare its image with the configured image. When they differ, stop the Container and start it with the configured image.

`inspect()` reports an empty image while a Container is starting and for a Container restored from a snapshot. The following example does not replace a Container in either case:

*src/index.jsjs*

```js
const image = this.ctx.container.images.base;
const info = await this.ctx.container.inspect();

if (info && info.image !== "" && info.image !== image) {
	await this.ctx.container.destroy();
}

if (!this.ctx.container.running) {
	this.ctx.container.start({
		image,
		instance: "standard-2",
		enableInternet: false,
	});
}
```

*src/index.tsts*

```ts
const image = this.ctx.container.images.base;
const info = await this.ctx.container.inspect();

if (info && info.image !== "" && info.image !== image) {
	await this.ctx.container.destroy();
}

if (!this.ctx.container.running) {
	this.ctx.container.start({
		image,
		instance: "standard-2",
		enableInternet: false,
	});
}
```

Caution

`destroy()` stops the Container immediately. In-progress work and any filesystem changes made since startup are lost. To preserve the filesystem, create a [snapshot](https://developers.cloudflare.com/containers/guides/snapshots/) before you stop the Container. A snapshot is tied to the image it was created from, so you cannot restore it onto the updated image.

### Use an external image

Note

The `durable_object` scheduling policy does not support direct image references from external registries. A named `image` source must use a digest-pinned reference from the Cloudflare managed registry.

To use an image from another registry, pull it and [push it to the Cloudflare managed registry](#use-images-from-other-registries). Then, [get its digest](#get-the-digest-of-a-pushed-image) and configure the resulting digest-pinned reference as a named image.

A named `dockerfile` entry can use an external image in its `FROM` instruction. Authenticate your local Docker installation before deploying if the base image is private.

## Use images with the `default` scheduling policy

Set the singular `image` field to a local Dockerfile or a supported registry reference. Changing this field deploys the new application image through an [application-wide rollout](https://developers.cloudflare.com/containers/configuration/rollouts/).

### Push images during `wrangler deploy`

When running `wrangler deploy`, if you set the `image` attribute in your [Wrangler configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#containers) to a path to a Dockerfile, Wrangler will build your container image locally using Docker, then push it to a registry run by Cloudflare. This registry is integrated with your Cloudflare account and is backed by [R2](https://developers.cloudflare.com/r2/). All authentication is handled automatically by Cloudflare both when pushing and pulling images.

Just provide the path to your Dockerfile:

```jsonc
{
	"containers": [
		{
			"image": "./Dockerfile"
		}
	]
}
```

```toml
[[containers]]
image = "./Dockerfile"
```

And deploy your Worker with `wrangler deploy`. No other image management is necessary.

On subsequent deploys, Wrangler will only push image layers that have changed, which saves space and time.

Note

Docker or a Docker-compatible CLI tool must be running for Wrangler to build and push images. This is not necessary if you are using a pre-built image, as described below.

### Use pre-built container images

With the `default` scheduling policy, Containers support images from the Cloudflare managed registry at `registry.cloudflare.com`, [Docker Hub ↗︎](https://hub.docker.com/), [Amazon ECR ↗︎](https://aws.amazon.com/ecr/), and [Google Artifact Registry ↗︎](https://cloud.google.com/artifact-registry).

Note

Cloudflare does not cache images pulled from Docker Hub, Amazon ECR, or Google Artifact Registry.

Docker Hub pulls may be subject to Docker Hub pull limits or fair-use restrictions. Pulling images from Amazon ECR or Google Artifact Registry may incur cloud provider egress charges.

Note

The `durable_object` scheduling policy does not pull from Docker Hub, Amazon ECR, or Google Artifact Registry, and does not use registry credentials configured with `wrangler containers registries configure`. Refer to [Use an external image](#use-an-external-image).

#### Use public Docker Hub images

To use a public Docker Hub image, set `image` to a fully qualified Docker Hub image reference in your Wrangler configuration.

For example:

```jsonc
{
	"containers": [
		{
			"image": "docker.io/<NAMESPACE>/<REPOSITORY>:<TAG>"
		}
	]
}
```

```toml
[[containers]]
image = "docker.io/<NAMESPACE>/<REPOSITORY>:<TAG>"
```

Public Docker Hub images do not require registry configuration.

Private Docker Hub images use the private registry configuration flow described next.

If Docker Hub credentials have been configured, those credentials are used to pull both public and private images.

Note

Official Docker Hub images use the `library` namespace. For example, use `docker.io/library/<IMAGE>:<TAG>` instead of `docker.io/<IMAGE>:<TAG>`.

#### Configure private registry credentials

To use a private image from Docker Hub, Amazon ECR, or Google Artifact Registry, run [`wrangler containers registries configure`](https://developers.cloudflare.com/workers/wrangler/commands/containers/#containers-registries-configure) for the registry domain.

Wrangler prompts for the secret and stores it in [Secrets Store](https://developers.cloudflare.com/secrets-store). If you do not already have a Secrets Store store, Wrangler prompts you to create one first.

Use `--secret-name` to name or reuse a secret, `--secret-store-id` to target a specific Secrets Store store, and `--skip-confirmation` for non-interactive runs. In CI or scripts, pass the secret through `stdin`.

#### Use private Docker Hub images

Configure Docker Hub in Wrangler using these values:

- registry domain: `docker.io`
- username flag: `--dockerhub-username=<YOUR_DOCKERHUB_USERNAME>`
- secret: Docker Hub personal access token with read-only access

To create a Docker Hub personal access token:

1. Sign in to [Docker Home ↗︎](https://app.docker.com/).
2. Go to **Account settings** > **Personal access tokens**.
3. Select **Generate new token**.
4. Give the token **Read** access, then copy the token value.

Interactive:

npmyarnpnpm

```
npx wrangler containers registries configure docker.io --dockerhub-username=<YOUR_DOCKERHUB_USERNAME>
```

```
yarn wrangler containers registries configure docker.io --dockerhub-username=<YOUR_DOCKERHUB_USERNAME>
```

```
pnpm wrangler containers registries configure docker.io --dockerhub-username=<YOUR_DOCKERHUB_USERNAME>
```

CI or scripts:

```bash
printf '%s' "$DOCKERHUB_PAT" | npx wrangler containers registries configure docker.io --dockerhub-username=<YOUR_DOCKERHUB_USERNAME> --secret-name=<SECRET_NAME> --skip-confirmation
```

After you configure the registry, use the same fully qualified Docker Hub image reference shown above.

#### Use private Amazon ECR images

Configure Amazon ECR in Wrangler using these values:

- registry domain: `<AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com`
- access key flag: `--aws-access-key-id=<AWS_ACCESS_KEY_ID>`
- secret: matching AWS secret access key

Public ECR images are not supported. To generate the required credentials, create an IAM user with a read-only policy. The following example grants access to all image repositories in AWS account `123456789012` in `us-east-1`.

```json
{
	"Version": "2012-10-17",
	"Statement": [
		{
			"Action": ["ecr:GetAuthorizationToken"],
			"Effect": "Allow",
			"Resource": "*"
		},
		{
			"Effect": "Allow",
			"Action": [
				"ecr:BatchCheckLayerAvailability",
				"ecr:GetDownloadUrlForLayer",
				"ecr:BatchGetImage"
			],
			// arn:${Partition}:ecr:${Region}:${Account}:repository/${Repository-name}
			"Resource": [
				"arn:aws:ecr:us-east-1:123456789012:repository/*"
				// "arn:aws:ecr:us-east-1:123456789012:repository/example-repo"
			]
		}
	]
}
```

After you create the IAM user, use its credentials to [configure the registry in Wrangler](https://developers.cloudflare.com/workers/wrangler/commands/containers/#containers-registries-configure). Wrangler prompts you to create a Secrets Store store if one does not already exist, then stores the secret there.

Interactive:

npmyarnpnpm

```
npx wrangler containers registries configure <AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com --aws-access-key-id=<AWS_ACCESS_KEY_ID>
```

```
yarn wrangler containers registries configure <AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com --aws-access-key-id=<AWS_ACCESS_KEY_ID>
```

```
pnpm wrangler containers registries configure <AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com --aws-access-key-id=<AWS_ACCESS_KEY_ID>
```

CI or scripts:

```bash
printf '%s' "$AWS_SECRET_ACCESS_KEY" | npx wrangler containers registries configure <AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com --aws-access-key-id=<AWS_ACCESS_KEY_ID> --secret-name=<SECRET_NAME> --skip-confirmation
```

After you configure the registry, use the fully qualified Amazon ECR image reference in your Wrangler configuration:

```jsonc
{
	"containers": [
		{
			"image": "<AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com/<REPOSITORY>:<TAG>"
		}
	]
}
```

```toml
[[containers]]
image = "<AWS_ACCOUNT_ID>.dkr.ecr.<AWS_REGION>.amazonaws.com/<REPOSITORY>:<TAG>"
```

#### Use private Google Artifact Registry images

Configure Google Artifact Registry in Wrangler using these values:

- registry domain: `<REGION>-docker.pkg.dev`
- Google service account email flag: `--gar-email=<SERVICE_ACCOUNT_EMAIL>`
- secret: the service account JSON key

The public credential is the service account email, supplied with `--gar-email`. It must match the `client_email` field in the service account key.

The private credential is the service account JSON key. Provide it through `stdin` (a file path, raw JSON, or base64) or the interactive prompt (a file path or base64). Wrangler stores the key base64-encoded in Secrets Store.

Caution

Only `*-docker.pkg.dev` hosts are supported. Container Registry hosts such as `gcr.io` and `*.gcr.io` are not supported, because Google has shut down Container Registry.

To generate the required credentials, create a service account with the **Artifact Registry Reader** role and download its JSON key:

1. In the [Google Cloud console ↗︎](https://console.cloud.google.com), go to **IAM & Admin** > **Service Accounts**.
2. Select **Create service account**, then enter a name, ID, and optional description.
3. Grant the service account the **Artifact Registry Reader** role, then select **Done**.
4. Select the service account, then open the **Keys** tab.
5. Select **Add key** > **Create new key**, choose **JSON**, then select **Create**. The key file downloads to your machine.

Interactive: Wrangler prompts for the key, where you enter a file path or base64-encoded JSON:

npmyarnpnpm

```
npx wrangler containers registries configure <REGION>-docker.pkg.dev --gar-email=<SERVICE_ACCOUNT_EMAIL>
```

```
yarn wrangler containers registries configure <REGION>-docker.pkg.dev --gar-email=<SERVICE_ACCOUNT_EMAIL>
```

```
pnpm wrangler containers registries configure <REGION>-docker.pkg.dev --gar-email=<SERVICE_ACCOUNT_EMAIL>
```

CI or scripts: Pipe the key through `stdin` (the key contents as raw JSON or base64, or a path to the key file)

```bash
cat <PATH_TO_KEY> | npx wrangler containers registries configure <REGION>-docker.pkg.dev --gar-email=<SERVICE_ACCOUNT_EMAIL> --secret-name=<SECRET_NAME> --skip-confirmation
```

If you have already stored the key in Secrets Store, reference the existing secret and omit the key:

```bash
npx wrangler containers registries configure <REGION>-docker.pkg.dev --gar-email=<SERVICE_ACCOUNT_EMAIL> --secret-name=<EXISTING_SECRET_NAME> --skip-confirmation
```

After you configure the registry, use the fully qualified Google Artifact Registry image reference in your Wrangler configuration:

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "containers": [
    {
      "image": "<REGION>-docker.pkg.dev/<PROJECT_ID>/<REPOSITORY>/<IMAGE>:<TAG>"
    }
  ]
}
```

```toml
[[containers]]
image = "<REGION>-docker.pkg.dev/<PROJECT_ID>/<REPOSITORY>/<IMAGE>:<TAG>"
```

To use an image from a registry not listed above, [push it to the Cloudflare Registry](#use-images-from-other-registries).

Note

With `wrangler dev`, image references from the Cloudflare Registry, Docker Hub, Amazon ECR, and Google Artifact Registry are supported in local development.

With `vite dev`, image references from external registries such as Docker Hub, Amazon ECR, and Google Artifact Registry are supported, but `vite dev` cannot pull directly from the Cloudflare Registry.

If you use a private Docker Hub, Amazon ECR, or Google Artifact Registry image in local development, authenticate to that registry locally, for example with `docker login`.

## Push images to the Cloudflare Registry

Both scheduling policies can use images that you push to the Cloudflare Registry yourself. With the `durable_object` policy, this is required for any image from Docker Hub, Amazon ECR, Google Artifact Registry, or another external registry.

### Use images from other registries

If you want to use a pre-built image from another registry provider, first make sure it exists locally, then push it to the Cloudflare Registry:

```bash
docker pull <PUBLIC_IMAGE>
docker tag <PUBLIC_IMAGE> <IMAGE>:<TAG>
```

Wrangler provides a command to push images to the Cloudflare Registry:

npmyarnpnpm

```
npx wrangler containers push <IMAGE>:<TAG>
```

```
yarn wrangler containers push <IMAGE>:<TAG>
```

```
pnpm wrangler containers push <IMAGE>:<TAG>
```

Or, you can use the `-p` flag with `wrangler containers build` to build and push an image in one step:

npmyarnpnpm

```
npx wrangler containers build -p -t <TAG> .
```

```
yarn wrangler containers build -p -t <TAG> .
```

```
pnpm wrangler containers build -p -t <TAG> .
```

This will output an image registry URI. With the `default` scheduling policy, you can use this tag reference in your Wrangler configuration:

```jsonc
{
	"containers": [
		{
			"image": "registry.cloudflare.com/<YOUR_ACCOUNT_ID>/<IMAGE>:<TAG>"
		}
	]
}
```

```toml
[[containers]]
image = "registry.cloudflare.com/<YOUR_ACCOUNT_ID>/<IMAGE>:<TAG>"
```

With the `durable_object` scheduling policy, a named `image` source requires a digest-pinned reference. [Get the digest of the pushed image](#get-the-digest-of-a-pushed-image) first.

### Get the digest of a pushed image

After you push an image, list its repository digests with Docker:

```bash
docker image inspect --format '{{join .RepoDigests "\n"}}' registry.cloudflare.com/<YOUR_ACCOUNT_ID>/<IMAGE>:<TAG>
```

Use the entry that starts with `registry.cloudflare.com/` as a named `image` source:

```jsonc
{
	"containers": [
		{
			"class_name": "AgentComputer",
			"scheduling_policy": "durable_object",
			"images": {
				"tools": {
					"image": "registry.cloudflare.com/<YOUR_ACCOUNT_ID>/<IMAGE>@sha256:<DIGEST>",
				},
			},
		},
	],
}
```

```toml
[[containers]]
class_name = "AgentComputer"
scheduling_policy = "durable_object"

[containers.images.tools]
image = "registry.cloudflare.com/<YOUR_ACCOUNT_ID>/<IMAGE>@sha256:<DIGEST>"
```

### Push images with CI

To use an image built in a continuous integration environment, install `wrangler` then build and push images using either `wrangler containers build` with the `--push` flag, or using the `wrangler containers push` command. With the `durable_object` scheduling policy, [get the digest of the pushed image](#get-the-digest-of-a-pushed-image) before you update your Wrangler configuration.

## Registry limits

Images are limited in size by available disk of the configured [instance type](https://developers.cloudflare.com/containers/platform/limits/#instance-types) for a Container.

Delete images with `wrangler containers images delete` to free up space, but reverting a Worker to a previous version that uses a deleted image will then error.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/image-management/#page","headline":"Image Management","description":"Configure and update Container images with the durable_object and default scheduling policies.","url":"https://developers.cloudflare.com/containers/guides/image-management/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/image-management/og.png?v=3fd9a8c2d3fb3253","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Learn how to run Container-enabled Workers locally with `wrangler dev` and `vite dev`.
title: Local Development
image: https://developers.cloudflare.com/containers/guides/local-dev/og.png?v=a2f82e93548ea4bd
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Local Development

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/local-dev/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

You can run both your container and your Worker locally by running [`npx wrangler dev`](https://developers.cloudflare.com/workers/wrangler/commands/workers/#dev) (or `vite dev` for Vite projects using the [Cloudflare Vite plugin](https://developers.cloudflare.com/workers/vite-plugin/)) in your project's directory.

To develop Container-enabled Workers locally, you will need to first ensure that a Docker compatible CLI tool and Engine are installed. For instance, you could use [Docker Desktop ↗︎](https://docs.docker.com/desktop/) or [Colima ↗︎](https://github.com/abiosoft/colima).

Containers that use the [Durable Object scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy) require Wrangler 4.136.0 or later for local development. To run the [`cloudflare/debian-trixie` managed image](https://developers.cloudflare.com/containers/guides/image-management/#use-the-cloudflare-managed-image) locally, use Wrangler 4.141.0 or later.

When you start a dev session, your container image will be built or downloaded. If your [Wrangler configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#containers) sets the `image` attribute to a local path, the image will be built using the local Dockerfile. If the `image` attribute is set to an image reference, the image will be pulled from the referenced registry, such as the Cloudflare Registry, Docker Hub, Amazon ECR, or Google Artifact Registry.

Note

With `wrangler dev`, image references from the Cloudflare Registry, Docker Hub, Amazon ECR, and Google Artifact Registry are supported in local development.

With `vite dev`, image references from external registries such as Docker Hub, Amazon ECR, and Google Artifact Registry are supported, but `vite dev` cannot pull directly from the Cloudflare Registry.

If you use a private Docker Hub, Amazon ECR, or Google Artifact Registry image in local development, authenticate to that registry locally, for example with `docker login`.

As a workaround for Cloudflare Registry images, point `vite dev` at a local Dockerfile that uses `FROM <IMAGE_REFERENCE>`. Docker then pulls the base image during the local build.

Container instances will be launched locally when your Worker code calls to create a new container. Requests will then automatically be routed to the correct locally-running container.

When the dev session ends, all associated container instances should be stopped, but local images are not removed, so that they can be reused in subsequent builds.

Note

If your Worker app creates many container instances, your local machine may not be able to run as many containers concurrently as is possible when you deploy to Cloudflare.

Also, `max_instances` configuration option does not apply during local development.

Additionally, if you regularly rebuild containers locally, you may want to clear out old container images (using `docker image prune` or similar) to reduce disk used.

## FUSE support

Miniflare automatically grants local containers the Docker privileges required for Filesystem in Userspace (FUSE). This applies to `wrangler dev`, the Cloudflare Vite plugin, and direct Miniflare use.

Miniflare grants these privileges when the local Docker daemon runs inside a virtual machine (VM). This includes Docker engines on macOS and through Windows Subsystem for Linux (WSL). On Linux, Miniflare grants the privileges for local rootless Docker when `/dev/fuse` is available.

Rootful Docker on Linux does not support FUSE by default during local development. Miniflare does not grant FUSE privileges when the Docker daemon does not meet these conditions or cannot be inspected.

## Iterating on Container code

When you develop with Wrangler or Vite, your Worker's code is automatically reloaded each time you save a change, but code running within the container is not.

To rebuild your container with new code changes, you can hit the `[r]` key on your keyboard, which triggers a rebuild. Container instances will then be restarted with the newly built images.

You may prefer to set up your own code watchers and reloading mechanisms, or mount a local directory into the local container images to sync code changes. This can be done, but there is no built-in mechanism for doing so, and best-practices will depend on the languages and frameworks you are using in your container code.

## Differences after you deploy

Local development runs your container in Docker. A deployed container that uses the [Durable Object scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy) differs from a local container in the following ways. Code that works locally can fail after you deploy.

### The hostname is longer than a DNS label

The hostname of a container is the name it uses for itself, which the `hostname` command prints. Nothing routes traffic to it. In a deployed container, the hostname is 64 characters, one more than a DNS label allows. `/etc/hosts` does not list it, so it does not resolve. Locally, the hostname is the 12-character Docker container ID, and `/etc/hosts` lists it. Code that looks up its own hostname can fail after you deploy.

For example, `python3 -m http.server` and other servers built on the Python `http.server` module, such as `wsgiref`, exit on startup with this error:

```txt
UnicodeEncodeError: 'idna' codec can't encode characters in position 0-63: label too long
```

To serve files with Python, use `socketserver`, which does not look up the hostname:

```sh
python3 -c 'import http.server as h, socketserver as s; s.ThreadingTCPServer(("", 8000), h.SimpleHTTPRequestHandler).serve_forever()'
```

### Other Linux users keep root capabilities

Locally, Docker gives no capabilities to a process that runs as a user other than `root`. File permissions apply to that process. In a deployed container, every process has the same Linux capabilities as `root`, regardless of user. For more information, refer to the [`exec()` `user` option](https://developers.cloudflare.com/containers/api/durable-object-container/#exec).

## Troubleshooting

### Ports

Your Worker can connect to any port in the container.

`getTcpPort()` requires a process listening on that port. If the process has not started yet, you will see the following error:

```txt
Container is not listening to port 8080
```

Retry until the port becomes available.

### Socket configuration - `internal error`

If you see an opaque `internal error` when attempting to connect to your container, you may need to set the `DOCKER_HOST` environment variable to the socket path your container engine is listening on. Wrangler or Vite will attempt to automatically find the correct socket to use to communicate with your container engine, but if that does not work, you may have to set this environment variable to the appropriate socket path.

### SSL errors with the Cloudflare One Client or a VPN

If you are running the Cloudflare One Client or a VPN that performs TLS inspection, HTTPS requests made during the Docker build process may fail with SSL or certificate errors. This happens because the VPN intercepts HTTPS traffic and re-signs it with its own certificate authority, which Docker does not trust by default.

To resolve this, you can either:

- Disable the Cloudflare One Client or your VPN while running `wrangler dev` or `wrangler deploy`, then re-enable it afterwards.
- Add the certificate to your Docker build context. The Cloudflare One Client exposes its certificate via the `NODE_EXTRA_CA_CERTS` and `SSL_CERT_FILE` environment variables on your host machine. You can pass the certificate into your Docker build as an environment variable, so that it is available during the build without being baked into the final image.

  ```dockerfile
  RUN if [ -n "$SSL_CERT_FILE" ]; then \
      cp "$SSL_CERT_FILE" /usr/local/share/ca-certificates/Custom_CA.crt && \
      update-ca-certificates; \
      fi
  ```

  Note

  The above Dockerfile snippet is an example. Depending on your base image, the commands to install certificates may differ (for example, Alpine uses `apk add ca-certificates` and a different certificate path).

  This snippet will store the certificate into the image. Depending on whether your production environment needs the certificate, you may choose to do this only during development or use it in production too.

  Wrangler invokes Docker automatically when you run `wrangler dev` or `wrangler deploy`, so if you need to pass build secrets, you will need to build and push the image manually using `wrangler containers push`.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/local-dev/#page","headline":"Local Development","description":"Learn how to run Container-enabled Workers locally with wrangler dev and vite dev.","url":"https://developers.cloudflare.com/containers/guides/local-dev/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/local-dev/og.png?v=a2f82e93548ea4bd","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Replace Container class lifecycle and routing helpers with direct container control inside a Durable Object.
title: Migrate from the Container class to the Durable Object Container API
image: https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/og.png?v=3000f6da75d6e8fa
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Migrate from the Container class to the Durable Object Container API

Last updated Sep 29, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

The Durable Object Container API lets your Durable Object coordinate container compute with persistent storage, alarms, and request handling. Access it as `this.ctx.container` inside a Durable Object with a container binding. When migrating from the `Container` class, replace its helpers with application code where needed.

Before changing code, identify which helpers your application uses. Some helpers require application code to preserve their existing behavior. For an API comparison, refer to [Choose an API](https://developers.cloudflare.com/containers/api/#choose-an-api).

You can migrate the implementation without replacing the Durable Object. Keep the Worker name, exported class name, binding, container image, and existing migration tags unchanged. Changing the TypeScript base class does not require a new Durable Object migration.

## Replace Container class helpers

The `Container` class extends `DurableObject`. Replace its inherited lifecycle and routing helpers with application code that uses `ctx.container`.

1. In your Worker, change the class to extend `DurableObject` from `cloudflare:workers`. Keep the exported class name to retain its existing container definition and Durable Object binding. Do not add a new Durable Object migration solely because you changed the base class. Refer to [Wrangler configuration](https://developers.cloudflare.com/containers/configuration/wrangler/).
2. Replace `start()` calls with [`ctx.container.start()`](https://developers.cloudflare.com/containers/api/durable-object-container/#start). Replace `stop()` calls with [`signal()`](https://developers.cloudflare.com/containers/api/durable-object-container/#signal) or [`destroy()`](https://developers.cloudflare.com/containers/api/durable-object-container/#destroy), as appropriate. Do not assume `start()` waits for a port to become ready.
3. Replace `defaultPort`, `containerFetch()`, and automatic `fetch()` routing with [`getTcpPort(port).fetch()`](https://developers.cloudflare.com/containers/api/durable-object-container/#gettcpport) and your own request routing. Check port readiness before forwarding requests.
4. Replace `sleepAfter` with [`setInactivityTimeout()`](https://developers.cloudflare.com/containers/api/durable-object-container/#setinactivitytimeout). Replace lifecycle hooks and `schedule()` with application code, [`monitor()`](https://developers.cloudflare.com/containers/api/durable-object-container/#monitor), and [Durable Object alarms](https://developers.cloudflare.com/durable-objects/api/alarms/) where appropriate.
5. Test startup, concurrent requests, readiness, idle shutdown, alarm delivery, storage continuity, and recovery after a container restart. A container can be temporarily unavailable after `stop()` or `destroy()`. Retry allocation before checking port readiness. Then remove `@cloudflare/containers` only if no other code imports it.

Keep the same image in the `containers` section of your Wrangler configuration unless you intend to change the container application itself. The direct API still runs the image associated with its Durable Object class.

For process handling and output, refer to [Execute commands](https://developers.cloudflare.com/containers/guides/execute-commands/) and the [`exec()` reference](https://developers.cloudflare.com/containers/api/durable-object-container/#exec).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/#page","headline":"Migrate from the Container class to the Durable Object Container API","description":"Replace Container class lifecycle and routing helpers with direct container control inside a Durable Object.","url":"https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/og.png?v=3000f6da75d6e8fa","dateModified":"2026-09-29","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Replace a Container application that uses the default scheduling policy with a Durable Object-managed application.
title: Migrate to the Durable Object scheduling policy
image: https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-scheduling-policy/og.png?v=c4a9b833bc8d835b
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Migrate to the Durable Object scheduling policy

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-scheduling-policy/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

You cannot change the scheduling policy of an existing Container application. To move from the `default` policy to the `durable_object` policy, create a replacement Container application and cut traffic over to it.

The replacement application must use a new Durable Object class and namespace. The old and replacement applications cannot attach to the same Durable Object namespace.

Caution

This process does not transfer existing Container instances or Durable Object storage. The `default` scheduling policy does not support snapshots, so you cannot use a Container snapshot to transfer a filesystem.

If the existing Durable Objects contain data, design an application-specific transfer process before continuing.

## Before you begin

- **Replace the `Container` class.** The [`Container` class](https://developers.cloudflare.com/containers/api/container-class/) does not support the `durable_object` policy. If your existing class extends `Container`, the replacement class must use the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/) directly. Rebuild any `Container` class helpers your application relies on, such as port readiness checks, request proxying, and sleep timeouts. Refer to [Migrate to the Durable Object Container API](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/).
- **Check how the Worker declares Durable Object classes.** A Worker uses either the [`exports` field](https://developers.cloudflare.com/durable-objects/reference/durable-objects-migrations/) or the [legacy `migrations` array](https://developers.cloudflare.com/durable-objects/reference/durable-object-class-migrations-legacy/), not both. Declare the replacement class with the same mechanism the Worker already uses. Moving from `migrations` to `exports` cannot be undone. If you want to move, do it as a separate change. Refer to [Migrate from the legacy `migrations` flow](https://developers.cloudflare.com/durable-objects/reference/durable-objects-migrations/#migrate-from-the-legacy-migrations-flow).
- **Move external images to the Cloudflare managed registry.** Named images with the `durable_object` policy must use a Dockerfile or a digest-pinned reference in the Cloudflare managed registry. If the existing `image` references Docker Hub, Amazon ECR, or Google Artifact Registry, [push the image to the Cloudflare managed registry](https://developers.cloudflare.com/containers/guides/image-management/#use-an-external-image) first.

## Configuration changes

Map the existing application configuration to its `durable_object` equivalent:

| `default` policy | `durable_object` policy |
| --- | --- |
| `image` (Dockerfile path) | `dockerfile` on a named `images` entry, selected with `ctx.container.start()` |
| `image` (registry reference) | `image` on a named `images` entry. Must be a digest-pinned reference in the Cloudflare managed registry |
| `image_build_context` | `build_context` on the named image |
| `image_vars` | `build_vars` on the named image |
| `instance_type` | The `instance` option in `ctx.container.start()`. Refer to [Instance size changes](#instance-size-changes) |
| `max_instances` | Not supported. Running instances count toward [account limits](https://developers.cloudflare.com/containers/platform/limits/#account-limits). Enforce any per-application cap in application code |
| Application-wide image rollouts | Application code that stops and starts each Container |
| `observability` | Keep in Wrangler configuration, subject to `durable_object` restrictions |
| `unsafe.configuration.experimental_flags` | Keep in Wrangler configuration |
| `ssh` and `authorized_keys` | Keep in Wrangler configuration |
| Placement constraints, rollout settings, `wrangler_ssh`, and `trusted_user_ca_keys` | Not supported on the replacement application |

For the complete field compatibility list, refer to [Wrangler configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#containers).

### Instance size changes

With the `durable_object` policy, you set the instance size with the `instance` option of `ctx.container.start()` instead of `instance_type` in Wrangler configuration. `instance` accepts `lite`, `standard-1`, `standard-2`, `standard-3`, and `standard-4`. If your existing application uses one of the following `instance_type` values, choose a replacement:

- `dev`: Use `lite`.
- `standard`: Use `standard-1`.
- `basic`: Choose `lite` or `standard-1`. Custom instance types require at least 1 vCPU, so you cannot reproduce `basic` with a custom instance.

Custom instance objects use camel case at runtime. Rename `memory_mib` to `memoryMib` and `disk_mb` to `diskMb`. Refer to [Choose an instance size at runtime](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#choose-an-instance-size-at-runtime).

## Move the application

1. **Add a new Durable Object class and Container application.**

   You cannot move the existing application by changing its `scheduling_policy` to `durable_object`, because the scheduling policy cannot be changed after creation. You also cannot reuse the existing Durable Object class, because each Durable Object namespace attaches to one Container application.

   Caution

   Do not change `scheduling_policy` on the existing Container entry. `wrangler deploy` deploys the new Worker version before it configures the Container application. The deploy then fails with the new Worker code already live and no `durable_object` application behind it.

   Instead, add all of the following to the Wrangler configuration:
   - A new SQLite-backed Durable Object class.
   - A Durable Object binding for the new class.
   - A new Container entry with `"scheduling_policy": "durable_object"`, a different `name`, and `class_name` set to the new class.

   Leave the existing Container entry, Durable Object class, and binding unchanged so that you can route traffic back to them. Changing the existing entry's `name` or `class_name` can cause Wrangler to create a new application instead of updating the existing one.

   The following example uses `exports`. If the Worker uses the legacy `migrations` array, add a new migration with `new_sqlite_classes: ["DurableSandbox"]` instead.

   ```jsonc
   {
       "name": "sandbox-worker",
       "main": "src/index.ts",
       "upload_source_maps": true,
       "compatibility_date": "2026-09-29",
       "containers": [
           // Existing application. Keep this entry unchanged.
           {
               "class_name": "Sandbox",
               "image": "./container/Dockerfile",
               "instance_type": "standard-2",
               "max_instances": 10,
           },
           // Replacement application.
           {
               "name": "sandbox-durable-object",
               "class_name": "DurableSandbox",
               "scheduling_policy": "durable_object",
               "images": {
                   "base": {
                       "dockerfile": "./container/Dockerfile",
                   },
               },
           },
       ],
       "durable_objects": {
           "bindings": [
               {
                   "name": "SANDBOX",
                   "class_name": "Sandbox",
               },
               {
                   "name": "DURABLE_SANDBOX",
                   "class_name": "DurableSandbox",
               },
           ],
       },
       "exports": {
           "Sandbox": {
               "type": "durable-object",
               "storage": "sqlite",
           },
           "DurableSandbox": {
               "type": "durable-object",
               "storage": "sqlite",
           },
       },
   }
   ```

   ```toml
   name = "sandbox-worker"
   main = "src/index.ts"
   upload_source_maps = true
   compatibility_date = "2026-09-29"

   [[containers]]
   class_name = "Sandbox"
   image = "./container/Dockerfile"
   instance_type = "standard-2"
   max_instances = 10

   [[containers]]
   name = "sandbox-durable-object"
   class_name = "DurableSandbox"
   scheduling_policy = "durable_object"

   [containers.images.base]
   dockerfile = "./container/Dockerfile"

   [[durable_objects.bindings]]
   name = "SANDBOX"
   class_name = "Sandbox"

   [[durable_objects.bindings]]
   name = "DURABLE_SANDBOX"
   class_name = "DurableSandbox"

   [exports.Sandbox]
   type = "durable-object"
   storage = "sqlite"

   [exports.DurableSandbox]
   type = "durable-object"
   storage = "sqlite"
   ```

2. **Move startup configuration into the replacement class.**

   Select the image and instance size when the replacement Durable Object starts its Container:

   *src/index.jsjs*

   

   ```js
   import { DurableObject } from "cloudflare:workers";

   export class DurableSandbox extends DurableObject {
   	startContainer() {
   		if (this.ctx.container.running) {
   			return;
   		}

   		this.ctx.container.start({
   			image: this.ctx.container.images.base,
   			instance: "standard-2",
   			// Match the existing application's outbound access.
   			enableInternet: true,
   		});
   	}
   }
   ```

   *src/index.tsts*

   

   ```ts
   import { DurableObject } from "cloudflare:workers";

   export class DurableSandbox extends DurableObject {
       startContainer() {
           if (this.ctx.container.running) {
               return;
           }

           this.ctx.container.start({
               image: this.ctx.container.images.base,
               instance: "standard-2",
               // Match the existing application's outbound access.
               enableInternet: true,
           });
       }
   }
   ```

   Move other supported startup settings, such as `env` and `entrypoint`, into the same call. Set `enableInternet` to match the existing application. The `Container` class allows outbound Internet access unless you set `enableInternet = false`.

   `ctx.container.start()` returns before the Container is ready to accept requests. Add an application-specific readiness check before sending traffic to the Container.
3. **Deploy and validate the replacement application.**

   Deploy both applications:npmyarnpnpm

   ```
   npx wrangler deploy
   ```

   ```
   yarn wrangler deploy
   ```

   ```
   pnpm wrangler deploy
   ```

   Start a replacement Container without changing production routing, for example through a test-only route that uses the `DURABLE_SANDBOX` binding. Confirm that the image, instance size, environment, entrypoint, network access, and readiness behavior match the existing application.
4. **Cut traffic over to the replacement namespace.**

   Update the Worker routing logic to resolve Container IDs through the replacement Durable Object binding. The same name resolves to a different Durable Object in each namespace. A Durable Object in the replacement namespace cannot access storage from the old namespace.

   Choose the binding per logical Container, not per request. If requests for the same name can reach both bindings, such as during a percentage-based rollout or a [gradual deployment](https://developers.cloudflare.com/workers/versions-and-deployments/gradual-deployments/), two Containers can run for one logical sandbox and their state can diverge. For example, record which namespace each sandbox uses and read that record when routing:

   ```ts
   // isMigrated() is application code that reads a per-sandbox record.
   const binding = (await isMigrated(sandboxName))
       ? env.DURABLE_SANDBOX
       : env.SANDBOX;
   const sandbox = binding.getByName(sandboxName);
   ```

   If Durable Object state must move, complete the application-specific transfer for each sandbox before routing it to the replacement namespace.
5. **Observe the replacement application.**

   Keep the old application, class, and binding during the observation period. To roll back, route traffic to the old Durable Object binding again.

   Writes made after cutover stay in the replacement namespace. If the application accepts writes, plan how to reconcile that data before rolling back.
6. **Delete the old Container application.**

   After the rollback period, remove the old Container entry from the Wrangler configuration and the old routing path from the Worker code. Deploy the updated Worker. Keep the old Durable Object class and binding until you no longer need its stored data.

   Removing the entry from Wrangler configuration does not delete the existing Container application. List the applications and copy the old application ID:npmyarnpnpm

   ```
   npx wrangler containers list
   ```

   ```
   yarn wrangler containers list
   ```

   ```
   pnpm wrangler containers list
   ```

   Delete the old application:npmyarnpnpm

   ```
   npx wrangler containers delete <OLD_APPLICATION_ID>
   ```

   ```
   yarn wrangler containers delete <OLD_APPLICATION_ID>
   ```

   ```
   pnpm wrangler containers delete <OLD_APPLICATION_ID>
   ```

   This command deletes the application and its Container instances.

   Delete the old Durable Object class only when its stored data is no longer needed. Deleting a Durable Object class permanently deletes its namespace and stored data. Refer to [Durable Object class exports](https://developers.cloudflare.com/durable-objects/reference/durable-objects-migrations/) or [Durable Object class migrations (legacy)](https://developers.cloudflare.com/durable-objects/reference/durable-object-class-migrations-legacy/) for the applicable cleanup process.

## Related resources

- [Scheduling Policies](https://developers.cloudflare.com/containers/configuration/scheduling-policy/)
- [Image Management](https://developers.cloudflare.com/containers/guides/image-management/)
- [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/)
- [Migrate to the Durable Object Container API](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-scheduling-policy/#page","headline":"Migrate to the Durable Object scheduling policy","description":"Replace a Container application that uses the default scheduling policy with a Durable Object-managed application.","url":"https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-scheduling-policy/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-scheduling-policy/og.png?v=c4a9b833bc8d835b","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Save and restore Container filesystems with the Durable Object scheduling policy.
title: Use snapshots
image: https://developers.cloudflare.com/containers/guides/snapshots/og.png?v=74144dfc758483da
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Use snapshots

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/snapshots/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Snapshots let you save point-in-time filesystem state from a running [Container](https://developers.cloudflare.com/containers/). The examples on this page use the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/).

Scheduling policy requirement

Snapshots are only supported by Container applications that use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy). Applications that use the `default` scheduling policy cannot create or restore snapshots.

## Create a container snapshot

Use `snapshotContainer()` to capture the full container filesystem. Snapshots are immutable. If you restore a snapshot and then change files, create a new snapshot to persist those changes.

Snapshot image compatibility

Snapshots capture the Container's complete filesystem state, but not its memory or running processes. A snapshot is tied to the Container image version it was created from and is not portable to a different image. After updating the image, create a new snapshot from a Container running that image.

The returned snapshot handle is a plain data object. You can store it and restore it later, including from another Durable Object:

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
	async saveContainer() {
		const containerSnapshot = await this.ctx.container.snapshotContainer({
			name: "before-upgrade",
		});

		await this.ctx.storage.put("containerSnapshot", containerSnapshot);
	}
}
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
	async saveContainer() {
		const containerSnapshot = await this.ctx.container.snapshotContainer({
			name: "before-upgrade",
		});

		await this.ctx.storage.put("containerSnapshot", containerSnapshot);
	}
}
```

## Restore a container snapshot

Load the saved snapshot handle. Then, pass it to `this.ctx.container.start()` when you start another container:

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
	async restoreContainer() {
		const containerSnapshot = await this.ctx.storage.get("containerSnapshot");

		if (!containerSnapshot) {
			return;
		}

		this.ctx.container.start({ containerSnapshot, enableInternet: false });
	}
}
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
	async restoreContainer() {
		const containerSnapshot =
			await this.ctx.storage.get<ContainerSnapshot>("containerSnapshot");

		if (!containerSnapshot) {
			return;
		}

		this.ctx.container.start({ containerSnapshot, enableInternet: false });
	}
}
```

## Understand retention

Snapshots have an implicit [30-day time-to-live](https://developers.cloudflare.com/containers/platform/limits/#snapshot-limits). Each restore refreshes that time-to-live.

You cannot set a custom time-to-live yet.

## Related resources

- [Scheduling Policies](https://developers.cloudflare.com/containers/configuration/scheduling-policy/) - Choose how Container instances are configured and updated
- [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/) - Full `ctx.container` API reference
- [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/) - Understand startup, sleep, and shutdown behavior

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/snapshots/#page","headline":"Use snapshots","description":"Save and restore Container filesystems with the Durable Object scheduling policy.","url":"https://developers.cloudflare.com/containers/guides/snapshots/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/snapshots/og.png?v=74144dfc758483da","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Connect to running container instances with SSH.
title: SSH
image: https://developers.cloudflare.com/containers/guides/ssh/og.png?v=4c9d3eebfeaea9f5
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# SSH

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/guides/ssh/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Anyone with write access to a Container can SSH into it with Wrangler as long as a matching public key is listed in `authorized_keys`.

SSH does not expose a publicly accessible port on the Container. The only way to connect is through Wrangler with [`wrangler containers ssh`](https://developers.cloudflare.com/workers/wrangler/commands/containers/#containers-ssh), which authenticates against your Cloudflare account.

## Configure SSH

SSH can be configured in your [Container's configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#containers) with the `ssh` and `authorized_keys` properties. Only the `ssh-ed25519` key type is supported.

The `ssh.enabled` property only controls whether you can SSH into a Container through Wrangler. It defaults to `true`. Set it to `false` to disable SSH access completely.

## Connect with Wrangler

To SSH into a Container with Wrangler, add an `ssh-ed25519` public key to `authorized_keys` in your Container configuration. The following example shows a basic configuration:

```jsonc
{
	"containers": [
		{
			// other options here...
			"authorized_keys": [
				{
					"name": "<NAME>",
					"public_key": "<YOUR_PUBLIC_KEY_HERE>"
				}
			]
		}
	]
}
```

```toml
[[containers]]
[[containers.authorized_keys]]
name = "<NAME>"
public_key = "<YOUR_PUBLIC_KEY_HERE>"
```

For more information on configuring SSH, refer to [SSH configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#ssh).

Find the instance ID for your Container by running [`wrangler containers instances`](https://developers.cloudflare.com/workers/wrangler/commands/containers/#containers-instances) or in the [Cloudflare dashboard ↗︎](https://dash.cloudflare.com/?to=/:account/workers/containers). The instance you want to SSH into must be running. SSH will not start a stopped Container, and an active SSH connection alone will not keep a Container alive.

Once SSH is configured and the Container is running, open the SSH connection with:

```bash
wrangler containers ssh <INSTANCE_ID>
```

To run a command instead of the default shell, pass it after `--`. For interactive commands, such as `top`, also pass `--tty` to allocate a terminal:

```bash
wrangler containers ssh <INSTANCE_ID> --tty -- top
```

## Use as SSH proxy

You can use `wrangler containers ssh` as an OpenSSH `ProxyCommand`. This lets your local SSH client connect through Wrangler.

```sh
ssh -o ProxyCommand="wrangler containers ssh %h" cloudchamber@<INSTANCE_ID>
```

When used this way, Wrangler pipes standard input and output to the SSH server in the running Container. You can also pass `--stdio` to force this mode.

## Process visibility

Without the [`containers_pid_namespace`](https://developers.cloudflare.com/workers/configuration/compatibility-flags/#use-an-isolated-pid-namespace-for-containers) compatibility flag, all processes inside the VM are visible when you connect to your Container through SSH. This flag is turned on by default for Workers with a [compatibility date](https://developers.cloudflare.com/workers/configuration/compatibility-dates/) of `2026-04-01` or later.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/guides/ssh/#page","headline":"SSH","description":"Connect to running container instances with SSH.","url":"https://developers.cloudflare.com/containers/guides/ssh/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/guides/ssh/og.png?v=4c9d3eebfeaea9f5","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Choose a scheduling policy, configure Containers in Wrangler, connect bindings, set environment variables, tune scaling, and manage rollouts.
title: Configuration
image: https://developers.cloudflare.com/containers/configuration/og.png?v=3f461e155b6684bc
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Configuration

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/configuration/#page","headline":"Configuration","description":"Choose a scheduling policy, configure Containers in Wrangler, connect bindings, set environment variables, tune scaling, and manage rollouts.","url":"https://developers.cloudflare.com/containers/configuration/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/og.png?v=3f461e155b6684bc","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Runtime and user-defined environment variables available inside Container instances.
title: Environment Variables
image: https://developers.cloudflare.com/containers/configuration/environment-variables/og.png?v=2ce5b2228960b5cc
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Environment Variables

Last updated Sep 29, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/environment-variables/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

## Runtime environment variables

The container runtime automatically sets the following variables:

- `CLOUDFLARE_APPLICATION_ID` - the ID of the Containers application
- `CLOUDFLARE_COUNTRY_A2` - the [ISO 3166-1 Alpha 2 code ↗︎](https://www.iso.org/obp/ui/#search/code/) of a country the container is placed in
- `CLOUDFLARE_LOCATION` - a name of a location the container is placed in
- `CLOUDFLARE_REGION` - a region name
- `CLOUDFLARE_DURABLE_OBJECT_ID` - the ID of the Durable Object instance that the container is bound to. You can use this to identify particular container instances on the dashboard.

## User-defined environment variables

You can set environment variables when defining a Container in your Worker, or when starting a container instance.

For example:

```javascript
class MyContainer extends Container {
	defaultPort = 4000;
	envVars = {
		MY_CUSTOM_VAR: "value",
		ANOTHER_VAR: "another_value",
	};
}
```

More details about defining environment variables and secrets can be found in [this example](https://developers.cloudflare.com/containers/examples/env-vars-and-secrets).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/environment-variables/#page","headline":"Environment Variables","description":"Runtime and user-defined environment variables available inside Container instances.","url":"https://developers.cloudflare.com/containers/configuration/environment-variables/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/environment-variables/og.png?v=2ce5b2228960b5cc","dateModified":"2026-09-29","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Intercept and handle outbound HTTP from containers using Workers.
title: Handle outbound traffic
image: https://developers.cloudflare.com/containers/configuration/outbound-traffic/og.png?v=2e06fa362fbb8548
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Handle outbound traffic

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/outbound-traffic/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Outbound handlers let you intercept and modify HTTP traffic from a container with trusted code.

Use them to:

- Allow or deny specific origin destinations
- Safely inject authorization headers or tokens
- Transparently reroute traffic
- Add custom policy on outbound traffic (such as denying specific HTTP requests)
- [Connect to Workers bindings](https://developers.cloudflare.com/containers/configuration/workers-connections/) like KV, R2, and Durable Objects

## Block outbound traffic

Use `enableInternet = false` to block public internet access by default:

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	enableInternet = false;
}
```

When `enableInternet` is `false`, only traffic that `allowedHosts` or an outbound handler allows can leave the container. Only ports `80` and `443` are available. Your outbound configuration decides which DNS lookups resolve. For more information, refer to [Non-HTTP traffic](#non-http-traffic).

Note

`enableInternet` takes effect when the container starts. Changes to `outbound` handlers and related outbound policies can affect a live-running container without restarting it.

## Block or allow traffic by host

You can filter outbound traffic with the `allowedHosts` and `deniedHosts` properties on the Container class.

Note

Export `ContainerProxy` from your Worker entrypoint for outbound interception to work.

When `allowedHosts` is set, it becomes a deny-by-default allowlist. Any host or IP not in the list is denied, and only matching destinations can reach `outbound` or `outboundByHost` handlers.

`allowedHosts` and `deniedHosts` also support simple glob patterns where `*` matches any sequence of characters.

By default, a Container will allow internet access, and you can set `deniedHosts` to disallow specific hosts or IPs:

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	// Make sure the container trusts /etc/cloudflare/certs/cloudflare-containers-ca.crt
	interceptHttps = true;
	deniedHosts = ["some-nefarious-website.com", "141.101.64.0/18"];
}
```

Caution

`deniedHosts` applies to HTTP on port `80`. With `interceptHttps = true`, it also applies to HTTPS on port `443`. While `enableInternet` is `true`, connections to other ports bypass `deniedHosts`. To block a destination on every port, set `enableInternet = false` and allow only what the container needs.

You can also disable internet access by default, but allow specific hosts and IPs:

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	// Make sure the container trusts /etc/cloudflare/certs/cloudflare-containers-ca.crt
	interceptHttps = true;

	// default internet access to off unless overridden by 'allowedHosts' or outbound proxy
	enableInternet = false;

	// overrides enableInternet = false
	allowedHosts = ["allowed.com"];
}
```

## Define outbound handlers

Outbound handlers are programmable egress proxies that run on the same machine as the container. They have access to all Workers bindings.

Use `outbound` to intercept HTTP requests on port `80` and HTTPS requests on port `443`:

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	interceptHttps = true;
}

MyContainer.outbound = async (request, env, ctx) => {
	if (request.method !== "GET") {
		console.log(`Blocked ${request.method} to ${request.url}`);
		return new Response("Method Not Allowed", { status: 405 });
	}
	return fetch(request);
};
```

Note

HTTP requests to the outbound handler remain secure because they run on the same machine as the container. You can upgrade requests to HTTPS from the Worker itself to prevent plain-text traffic from reaching the internet.

Use `outboundByHost` to map specific domain names or IP addresses to proxy functions:

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	interceptHttps = true;
}

MyContainer.outboundByHost = {
	"my.worker": async (request, env, ctx) => {
		// Run arbitrary Workers logic from this hostname
		return await someWorkersFunction(request.body);
	},
};
```

Calls to `http://my.worker` from the container invoke the handler, which runs inside the Workers runtime, outside the container sandbox.

`deniedHosts` and `allowedHosts` are evaluated before any outbound handler. If you use `allowedHosts`, include the hostname there for either `outbound` or `outboundByHost` to run. `outboundByHost` handlers take precedence over catch-all `outbound` handlers.

## Securely inject credentials

Because outbound handlers run in the Workers runtime — outside the container sandbox — they can hold secrets that the container itself never sees. The container makes a plain HTTP request, and the handler attaches the credential before forwarding it to the upstream service.

```js
export class MyContainer extends Container {
	// Make sure the container trusts /etc/cloudflare/certs/cloudflare-containers-ca.crt
	interceptHttps = true;
}

MyContainer.outboundByHost = {
	"github.com": (request, env, ctx) => {
		const requestWithAuth = new Request(request);
		requestWithAuth.headers.set("x-auth-token", env.SECRET);
		return fetch(requestWithAuth);
	},
};
```

This is especially useful for agentic workloads where you cannot fully trust the code running inside the container. With this pattern:

- **No token is exposed to the container.** The secret lives in the Worker's environment and is never passed into the sandbox.
- **No token rotation inside the container.** Rotate the secret in your Worker's environment and every request picks it up immediately.
- **Per-host and per-instance rules.** Combine `outboundByHost` with `ctx.containerId` to scope credentials or permissions to a specific container instance.

Here, `ctx.containerId` looks up a per-instance key from KV:

```js
export class MyContainer extends Container {
	// Make sure the container trusts /etc/cloudflare/certs/cloudflare-containers-ca.crt
	interceptHttps = true;
}

MyContainer.outboundByHost = {
	"my-internal-vcs.dev": async (request, env, ctx) => {
		const authKey = await env.KEYS.get(ctx.containerId);

		const requestWithAuth = new Request(request);
		requestWithAuth.headers.set("x-auth-token", authKey);
		return fetch(requestWithAuth);
	},
};
```

## HTTPS traffic

By default, HTTPS traffic is not intercepted by outbound handlers. To opt in you must set the `interceptHttps` attribute.

```js
export class MyContainer extends Container {
	// Make sure the container trusts /etc/cloudflare/certs/cloudflare-containers-ca.crt
	interceptHttps = true;
}

MyContainer.outbound = (req, env, ctx) => {
	// All HTTP(S) requests will trigger this hook.
	return fetch(req);
};
```

This is useful for Sandbox-like services that redirect untrusted traffic from a container instance to Workers for filtering and modification.

When HTTPS interception is active, an ephemeral CA file will be created at `/etc/cloudflare/certs/cloudflare-containers-ca.crt` once your container starts. The CA is only injected when you both set `interceptHttps = true` and define an `outbound` or `outboundByHost` handler.

### Trust the CA certificate

For HTTPS interception to work, you must trust the CA file. The CA is ephemeral and only exists at runtime, so do not try to bake it into your image during `docker build`. Instead, copy it into your distro's trust store and refresh the trust store from the container `entrypoint` before your application starts.

If your base image does not already include the trust-store tooling, install the distro's `ca-certificates` package in your image first.

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	interceptHttps = true;
	entrypoint = [
		"sh",
		"-lc",
		[
			"cp /etc/cloudflare/certs/cloudflare-containers-ca.crt /usr/local/share/ca-certificates/cloudflare-containers-ca.crt",
			"update-ca-certificates",
			"exec node server.js",
		].join(" && "),
	];
}
```

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	interceptHttps = true;
	entrypoint = [
		"sh",
		"-lc",
		[
			"cp /etc/cloudflare/certs/cloudflare-containers-ca.crt /usr/local/share/ca-certificates/cloudflare-containers-ca.crt",
			"update-ca-certificates",
			"exec node server.js",
		].join(" && "),
	];
}
```

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	interceptHttps = true;
	entrypoint = [
		"sh",
		"-lc",
		[
			"cp /etc/cloudflare/certs/cloudflare-containers-ca.crt /etc/pki/ca-trust/source/anchors/cloudflare-containers-ca.crt",
			"update-ca-trust",
			"exec node server.js",
		].join(" && "),
	];
}
```

```js
import { Container, ContainerProxy } from "@cloudflare/containers";
export { ContainerProxy };

export class MyContainer extends Container {
	interceptHttps = true;
	entrypoint = [
		"sh",
		"-lc",
		[
			"cp /etc/cloudflare/certs/cloudflare-containers-ca.crt /etc/ca-certificates/trust-source/anchors/cloudflare-containers-ca.crt",
			"trust extract-compat",
			"exec node server.js",
		].join(" && "),
	];
}
```

Replace `node server.js` with the command that starts your application.

Most runtimes will then trust the CA through the system root store automatically. If your runtime uses its own CA bundle, point it at `/etc/cloudflare/certs/cloudflare-containers-ca.crt` directly, for example with `NODE_EXTRA_CA_CERTS` or `REQUESTS_CA_BUNDLE`.

Note

HTTP communication to the outbound handler is encrypted by the networking stack. For traffic that stays within the Cloudflare Developer Platform, plain HTTP is secure.

## Non-HTTP traffic

Outbound handlers only intercept HTTP and HTTPS traffic. Traffic on ports other than `80` and `443` is never routed through `outbound` or `outboundByHost`.

If you set `enableInternet = false`, that traffic is denied. DNS lookups never reach a public resolver, so DNS cannot carry data out of the container. The container network answers lookups only for hostnames that your outbound configuration covers. Each answer is a placeholder address that routes the request to your Worker. The properties you set decide which hostnames resolve:

- With `allowedHosts`, `deniedHosts`, or `outbound` set, every hostname resolves, and your Worker allows or blocks each request.
- With only `outboundByHost` set, only its hostnames resolve.

Other lookups time out. For the exact rules, refer to [`interceptOutboundHttp`](https://developers.cloudflare.com/containers/api/durable-object-container/#interceptoutboundhttp).

## Change policies at runtime

Use `outboundHandlers` to define named handlers, then assign them to specific hosts at runtime using `setOutboundByHost()`. You can also apply a handler globally with `setOutboundHandler()`.

You can also manage runtime policy with `setOutboundByHosts()`, `setAllowedHosts()`, `setDeniedHosts()`, `allowHost()`, `denyHost()`, `removeAllowedHost()`, and `removeDeniedHost()`.

This lets a trusted Worker hold credentials without exposing them to an untrusted container:

```js
export class MyContainer extends Container {
	// Make sure the container trusts /etc/cloudflare/certs/cloudflare-containers-ca.crt
	interceptHttps = true;
}

MyContainer.outboundHandlers = {
	authenticatedGithub: async (request, env, ctx) => {
		const githubToken = env.GITHUB_TOKEN;
		return authenticateGitHttpsRequest(request, githubToken, ctx.containerId);
	},
};
```

Apply handlers to hosts programmatically from your Worker:

```js
async setUpContainer(req, env) {
  const container = await env.MY_CONTAINER.getByName("my-instance");

  // Give the container access to github.com on a specific host during setup
  await container.setOutboundByHost("github.com", "authenticatedGithub");

	// do something with github.com on your container...
}

async removeAccessToGithub(req, env) {
  const container = await env.MY_CONTAINER.getByName("my-instance");

  // Remove access to Github
  await container.removeOutboundByHost("github.com");
}
```

## Handler precedence

Requests are evaluated in this order:

1. `deniedHosts` is checked first. Matching hosts or IPs are denied immediately.
2. `allowedHosts` is checked next. When it is set, any host or IP not in the list is denied. Matching hosts continue to outbound handlers, or egress to the public internet if no handler is set.
3. Instance-level rules set with `setOutboundByHost()` are checked before class-level `outboundByHost` rules.
4. Per-host handlers always take precedence over catch-all handlers, so `outboundByHost` runs before `outbound`.
5. Instance-level handlers set with `setOutboundHandler()` are checked before the class-level `outbound` handler.
6. If no handler matches, the request can still egress to the public internet when it matched `allowedHosts` or `enableInternet = true`. Otherwise, it is denied.

## Direct API

To configure outbound interception directly on `ctx.container`, use `interceptOutboundHttp` for a specific hostname glob, IP, or CIDR range, or `interceptAllOutboundHttp` for all HTTP on port `80`. Both accept a `WorkerEntrypoint`.

```js
import { WorkerEntrypoint } from "cloudflare:workers";

export class MyOutboundWorker extends WorkerEntrypoint {
	fetch(request) {
		// Inspect, modify, or deny the request before passing it on
		return fetch(request);
	}
}

// Inside your Container DurableObject
this.ctx.container.start({ enableInternet: false });
const worker = this.ctx.exports.MyOutboundWorker({ props: {} });
await this.ctx.container.interceptAllOutboundHttp(worker);
```

You can call these methods before or after starting the container, and even while connections are open. In-flight TCP connections pick up the new handler automatically — no connections are dropped.

```js
// Intercept a specific CIDR range
await this.ctx.container.interceptOutboundHttp("203.0.113.0/24", worker);
// Intercept by hostname
this.ctx.container.interceptOutboundHttp("foo.com", worker);

// Update the handler while the container is running
const updated = this.ctx.exports.MyOutboundWorker({
	props: { phase: "post-install" },
});
await this.ctx.container.interceptOutboundHttp("203.0.113.0/24", updated);
```

For HTTPS, `interceptOutboundHttps` works the same way as `interceptOutboundHttp`.

```js
// Intercept a specific hostname
this.ctx.container.interceptOutboundHttps("foo.com", worker);

// Intercept HTTPS to every hostname
this.ctx.container.interceptOutboundHttps("*", worker);
```

The `Container` class calls these methods automatically when you use the functions shown above. You can also call them directly for cases the class does not cover.

## Local development

`wrangler dev` supports outbound interception. A sidecar process is spawned inside the container's network namespace. It applies `TPROXY` rules to route matching traffic to the local Workerd instance, mirroring production behavior.

## Related resources

- [Connect to Workers bindings](https://developers.cloudflare.com/containers/configuration/workers-connections/) — Access KV, R2, Durable Objects, and other bindings from a container
- [Control outbound traffic (Sandboxes)](https://developers.cloudflare.com/sandbox/sdk/guides/outbound-traffic/) — Sandbox SDK API for outbound handlers
- [Environment variables and secrets](https://developers.cloudflare.com/containers/configuration/environment-variables/) — Configure secrets and environment variables
- [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/) — Full `ctx.container` API reference

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/outbound-traffic/#page","headline":"Handle outbound traffic","description":"Intercept and handle outbound HTTP from containers using Workers.","url":"https://developers.cloudflare.com/containers/configuration/outbound-traffic/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/outbound-traffic/og.png?v=2e06fa362fbb8548","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: How container instances update after a deploy, including step percentages, grace periods, and rollout modes.
title: Rollouts
image: https://developers.cloudflare.com/containers/configuration/rollouts/og.png?v=3d0c4ea5b7db40b1
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Rollouts

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/rollouts/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Note

Rollouts apply to Container applications that use the [`default` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/). Durable Object-managed Container instances do not participate in application-wide rollouts; application code selects their image when it calls `ctx.container.start()`.

## How rollouts work

A **rollout** applies a target container application configuration after you [deploy](https://developers.cloudflare.com/containers/guides/deploy/) a Worker that uses Containers. The target can change the image, instance type, limits, placement, or other container settings.

A **container instance** is one running copy of your container image on Cloudflare's network. It runs the process your image starts (`ENTRYPOINT`/`CMD` in the Dockerfile, or the base image default). When the target changes the image, the rollout replaces container instances with copies that run the target image. Rollouts do not change Durable Object storage.

When an existing container application's effective configuration changes, `wrangler deploy`:

1. Uploads and activates the new Worker version, including Durable Object class code.
2. Builds and pushes a Dockerfile image when needed, or uses the configured registry image reference.
3. Starts a rollout to apply the target container configuration.

The Worker is active before the image and rollout steps begin. These steps are not transactional, so the Worker can remain active if a later image or rollout step fails. Deploy success means the rollout started, not that every container instance has finished replacing. The first deploy creates the container application directly, and a deploy with no effective container changes starts no rollout.

When the image changes, new Worker code can still reach container instances on the previous image until the rollout finishes. Prefer Worker and image changes that work together during that window, or choose [immediate](#immediate) when you need the shortest mixed window the platform allows.

Field names and allowed values are listed under [Containers configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#containers).

## Defaults

| Setting | Default |
| --- | --- |
| `rollout_step_percentage` | `100` if `max_instances` is omitted or less than `2`; otherwise `[10, 100]` |
| `rollout_active_grace_period` | `0` seconds |
| Stop sequence when replacing a container instance | `SIGTERM` to the main process, then `SIGKILL` after 15 minutes if it has not exited |

## Gradual rollouts

By default, Wrangler starts a rolling rollout using `rollout_step_percentage`. If `max_instances` is omitted or less than `2`, Wrangler uses one `100` step. Otherwise, Wrangler requests `[10, 100]`:

1. Request a target of about 10% of container instances with the new configuration. The platform raises this percentage when necessary so the step represents at least one instance at the configured `max_instances`.
2. Target 100% of container instances with the new configuration.

Configure the steps with `rollout_step_percentage` in Wrangler. Override the default plan for one deploy with [`--containers-rollout`](#rollout-modes).

## How a container instance is replaced

When the rollout selects a container instance to update:

1. **Grace period (if configured).** If `rollout_active_grace_period` is greater than `0`, container instances that only recently became connected to their [Durable Object](https://developers.cloudflare.com/durable-objects/) are skipped until they pass that window. Default `0` means no extra wait. Refer to [Rollout active grace period](#rollout-active-grace-period).
2. **Signal stop.** The platform sends `SIGTERM` to the main process in the container so it can stop accepting new work and finish in-flight work. Handle `SIGTERM` in your image if that process needs cleanup before exit.
3. **Drain.** The process has up to 15 minutes to exit after `SIGTERM`.
4. **Force stop if needed.** If the process is still running after 15 minutes, the platform sends `SIGKILL`.
5. **After exit.** The Container class [`onStop`](https://developers.cloudflare.com/containers/api/container-class/#onstop) hook can run in the Worker once the container process has exited.
6. **Start a new container instance** with the target image. Disk is [ephemeral](https://developers.cloudflare.com/containers/faq/#is-disk-persistent-what-happens-to-my-disk-when-my-container-sleeps) unless you store data outside the container filesystem.

Each selected container instance follows this sequence on its own schedule. The fleet does not restart in a single moment.

### Requests while a container instance starts

The new container instance must start its process. Startup often takes on the order of seconds, depending on image size and what runs at start. Refer to [cold starts](https://developers.cloudflare.com/containers/concepts/architecture/#starting-a-container).

A request that needs that container instance may wait until the container is ready, or fail if a client or Worker timeout is shorter than startup. Keep startup work fast, use port readiness checks if you configure them, and set timeouts with startup in mind.

## Rollout active grace period

`rollout_active_grace_period` applies only during a rollout, when the platform chooses which container instances to replace.

Containers are [backed by Durable Objects](https://developers.cloudflare.com/containers/concepts/architecture/#worker-to-durable-object). Each running container instance is associated with a Durable Object instance that starts it and sends it traffic. The grace period is how long that connection must already have been up before a rollout may shut the container down. It is not measured from deploy completion.

| Value | Effect |
| --- | --- |
| `0` (default) | No extra protection. Selected container instances may be replaced as soon as the rollout reaches them. |
| Greater than `0` (for example `300`) | Container instances connected to their Durable Object for less than this many seconds are left alone until they pass the window. |

Use a non-zero value when short sessions should finish before a rollout replaces the container. Container instances that have been connected longer can still be replaced once they pass the window.

`rollout_active_grace_period` applies in every rollout mode, including [immediate](#immediate).

## Rollout modes

`--containers-rollout` applies to [`wrangler deploy`](https://developers.cloudflare.com/workers/wrangler/commands/workers/#deploy) only. It does not apply to [`wrangler versions upload`](https://developers.cloudflare.com/workers/wrangler/commands/workers/#versions).

On a full deploy, Wrangler activates the Worker before it processes the container image and rollout. Rollout mode controls how the target container configuration is applied.

| Mode | Flag | Container instances |
| --- | --- | --- |
| Gradual (default) | omit flag | Use `rollout_step_percentage`, which can contain one or multiple steps |
| Immediate | `--containers-rollout=immediate` | Target 100% of container instances in one step |
| None | `--containers-rollout=none` | Leave images and running container instances unchanged; deploy Worker code only |

### Immediate

Immediate sets the rollout plan to a single step that targets 100% of container instances. There is no intermediate percentage hold.

npmyarnpnpm

```
npx wrangler deploy --containers-rollout=immediate
```

```
yarn wrangler deploy --containers-rollout=immediate
```

```
pnpm wrangler deploy --containers-rollout=immediate
```

Use immediate when Worker code and the container image need to stay compatible and you want the mixed window as short as the platform allows (for example a breaking change in how the Worker talks to the process in the image).

Behavior:

- The new Worker version is activated before the container image and rollout are processed.
- The rollout then replaces container instances toward 100% using the same [replace sequence](#how-a-container-instance-is-replaced) as gradual mode, including grace period when configured.
- Replacements complete over wall-clock time. How long depends on how many container instances are running, how long each takes to stop and start, and any grace period.
- When the image changes, immediate minimizes but does not eliminate the period when the new Worker can reach instances on the previous image.
- Deploy success means the rollout started, not that replacements finished.

### None

None leaves images and running container instances unchanged and deploys Worker code only.

Use none when the deploy should not publish a new image or start a container instance rollout. If `image` is a Dockerfile path and Docker is unavailable, Wrangler may require this flag or a working Docker setup so the deploy can skip container steps.

## Example configuration

`rollout_active_grace_period` of 300 seconds (five minutes) and steps `[10, 100]`:

```jsonc
{
	"containers": [
		{
			"max_instances": 10,
			"class_name": "MyContainer",
			"image": "./Dockerfile",
			"rollout_active_grace_period": 300,
			"rollout_step_percentage": [10, 100],
		},
	],
	"durable_objects": {
		"bindings": [
			{
				"name": "MY_CONTAINER",
				"class_name": "MyContainer",
			},
		],
	},
	"exports": {
		"MyContainer": {
			"type": "durable-object",
			"storage": "sqlite",
		},
	},
}
```

```toml
[[containers]]
max_instances = 10
class_name = "MyContainer"
image = "./Dockerfile"
rollout_active_grace_period = 300
rollout_step_percentage = [ 10, 100 ]

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"
```

## Related

- [Deploy Containers](https://developers.cloudflare.com/containers/guides/deploy/)
- [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/)
- [Image management](https://developers.cloudflare.com/containers/guides/image-management/)
- [Wrangler configuration](https://developers.cloudflare.com/containers/configuration/wrangler/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/rollouts/#page","headline":"Rollouts","description":"How container instances update after a deploy, including step percentages, grace periods, and rollout modes.","url":"https://developers.cloudflare.com/containers/configuration/rollouts/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/rollouts/og.png?v=3d0c4ea5b7db40b1","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Scale Container instances using explicit IDs or the getRandom helper for stateless load balancing.
title: Scaling and Routing
image: https://developers.cloudflare.com/containers/configuration/scaling-and-routing/og.png?v=d0d420b171f18fca
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Scaling and Routing

Last updated Sep 29, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/scaling-and-routing/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

## Scale container instances with explicit IDs

Note

This section uses helpers from the [Container class](https://developers.cloudflare.com/containers/api/container-class/).

Today, Containers are scaled manually by getting containers with a unique ID, then starting the container. Note that getting a container does not automatically start it.

```typescript
// get and start two container instances
const containerOne = getContainer(
	env.MY_CONTAINER,
	idOne,
).startAndWaitForPorts();

const containerTwo = getContainer(
	env.MY_CONTAINER,
	idTwo,
).startAndWaitForPorts();
```

Each instance will run until its `sleepAfter` time has elapsed, or until it is manually stopped.

This behavior is very useful when you want explicit control over the lifecycle of container instances. For instance, you may want to spin up a container backend instance for a specific user, or you may briefly run a code sandbox to isolate AI-generated code, or you may want to run a short-lived batch job.

### Use the `getRandom` helper function

If you want to run multiple instances of a container and route requests between them, use the `getRandom` helper function:

```javascript
import { Container, getRandom } from "@cloudflare/containers";

const INSTANCE_COUNT = 3;

class Backend extends Container {
	defaultPort = 8080;
	sleepAfter = "2h";
}

export default {
	async fetch(request: Request, env: Env): Promise<Response> {
		const containerInstance = await getRandom(env.BACKEND, INSTANCE_COUNT);
		return containerInstance.fetch(request);
	},
};
```

Use `getRandom` to route to multiple stateless container instances. It randomly selects one of N instances for each request, which means:

- It requires that the user set a fixed number of instances to route to.
- It will randomly select each instance, regardless of location.

We plan to fix these issues with built-in autoscaling and routing features in the near future.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/scaling-and-routing/#page","headline":"Scaling and Routing","description":"Scale Container instances using explicit IDs or the getRandom helper for stateless load balancing.","url":"https://developers.cloudflare.com/containers/configuration/scaling-and-routing/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/scaling-and-routing/og.png?v=d0d420b171f18fca","dateModified":"2026-09-29","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Choose whether Container configuration is managed centrally or by each Durable Object at runtime.
title: Scheduling Policies
image: https://developers.cloudflare.com/containers/configuration/scheduling-policy/og.png?v=d30120184c1c529d
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Scheduling Policies

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/scheduling-policy/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

A scheduling policy determines where you configure a Container image and [instance size](https://developers.cloudflare.com/containers/platform/limits/#instance-types). It also determines how image updates and [rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/) apply. Choose the policy when you create the Container application.

| Policy | Configure image and instance size | Image updates | Best for |
| --- | --- | --- | --- |
| `default` | In Wrangler configuration | Cloudflare applies application-wide configuration changes with [rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/) | Services whose instances use the same image and instance size |
| `durable_object` (beta) | In Durable Object code when calling `ctx.container.start()` | Application code selects an image each time it starts an instance | Sandboxes, agent environments, and other workloads that need per-instance configuration |

Note

The `durable_object` scheduling policy is in public beta.

One Wrangler configuration can contain applications with both policies. This lets a Worker use centrally managed service Containers alongside Durable Object-managed sandboxes.

The scheduling policy is immutable. To use a different policy, create a new Container application. Deleting an application also deletes its Container instances, so switching policies means replacing every running instance. To move an existing application, refer to [Migrate to the Durable Object scheduling policy](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-scheduling-policy/).

## Use the default scheduling policy

With the `default` policy, Wrangler configuration defines one application-wide `image`, one [`instance_type`](https://developers.cloudflare.com/containers/platform/limits/#instance-types), and settings such as `max_instances`. Omitting `scheduling_policy` selects `default`.

```jsonc
{
	"containers": [
		{
			"class_name": "ApiContainer",
			"scheduling_policy": "default",
			"image": "./api/Dockerfile",
			"instance_type": "standard-1",
			"max_instances": 5,
		},
	],
}
```

```toml
[[containers]]
class_name = "ApiContainer"
scheduling_policy = "default"
image = "./api/Dockerfile"
instance_type = "standard-1"
max_instances = 5
```

When you change the image or instance type and deploy, Cloudflare rolls out that change across the application. Refer to [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/).

## Use the Durable Object scheduling policy

The `durable_object` policy moves per-instance decisions into your Durable Object. Wrangler associates the Container application with the Durable Object class, and your code supplies a startup image or snapshot and an optional instance size to `ctx.container.start()`.

Configure the policy. To use custom images, add the images that the Durable Object can start:

```jsonc
{
	"name": "agent-computer",
	"main": "src/index.ts",
	"upload_source_maps": true,
	"compatibility_date": "2026-09-29",
	"containers": [
		{
			"class_name": "AgentComputer",
			"scheduling_policy": "durable_object",
			"images": {
				"base": {
					"dockerfile": "./container/Dockerfile",
				},
			},
		},
	],
	"durable_objects": {
		"bindings": [
			{
				"name": "AGENT_COMPUTER",
				"class_name": "AgentComputer",
			},
		],
	},
	"exports": {
		"AgentComputer": {
			"type": "durable-object",
			"storage": "sqlite",
		},
	},
}
```

```toml
name = "agent-computer"
main = "src/index.ts"
upload_source_maps = true
compatibility_date = "2026-09-29"

[[containers]]
class_name = "AgentComputer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./container/Dockerfile"

[[durable_objects.bindings]]
name = "AGENT_COMPUTER"
class_name = "AgentComputer"

[exports.AgentComputer]
type = "durable-object"
storage = "sqlite"
```

Wrangler builds or resolves each named image, prepares it for the Containers runtime, and exposes its digest-pinned reference on `ctx.container.images`. Select an image and instance size when the Durable Object starts its Container:

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class AgentComputer extends DurableObject {
	startContainer() {
		if (this.ctx.container.running) {
			return;
		}

		this.ctx.container.start({
			image: this.ctx.container.images.base,
			instance: "standard-2",
			enableInternet: false,
		});
	}
}
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

export class AgentComputer extends DurableObject {
	startContainer() {
		if (this.ctx.container.running) {
			return;
		}

		this.ctx.container.start({
			image: this.ctx.container.images.base,
			instance: "standard-2",
			enableInternet: false,
		});
	}
}
```

`ctx.container.start()` initiates startup and returns before the Container is ready to accept requests. Add an application-specific readiness check before sending traffic or calling [`exec()`](https://developers.cloudflare.com/containers/api/durable-object-container/#exec).

The `durable_object` policy does not support `max_instances`. Running instances count toward your [account limits](https://developers.cloudflare.com/containers/platform/limits/#account-limits).

### Use the Cloudflare-managed image

If you do not need a custom image, start the [`cloudflare/debian-trixie` Cloudflare-managed image](https://developers.cloudflare.com/containers/guides/image-management/#use-the-cloudflare-managed-image) directly without adding a named image to Wrangler. It includes Node.js 24.20.0 on Debian Trixie slim:

*src/index.jsjs*

```js
this.ctx.container.start({
	image: "cloudflare/debian-trixie",
	instance: "standard-2",
	enableInternet: false,
});
```

*src/index.tsts*

```ts
this.ctx.container.start({
	image: "cloudflare/debian-trixie",
	instance: "standard-2",
	enableInternet: false,
});
```

### Configure named images

Each key under `containers[].images` in your Wrangler configuration becomes a property on `ctx.container.images`. For example, `containers[].images.base` is available to the Durable Object as `ctx.container.images.base`. Each value must specify exactly one image source. Use `dockerfile` for a path to a Dockerfile. When you run `wrangler deploy`, Wrangler builds and uploads that image. You can also set `build_context` and `build_vars` for that image.

Use `image` for a digest-pinned image in the Cloudflare managed registry. Use the form `registry.cloudflare.com/<ACCOUNT_ID>/<REPOSITORY>@sha256:<DIGEST>`. Direct references to Docker Hub, Amazon ECR, or Google Artifact Registry are not supported. To use an image from another registry, [push it to the Cloudflare managed registry](https://developers.cloudflare.com/containers/guides/image-management/#use-an-external-image) first.

A configuration can contain up to 100 named images. An image name must contain between 1 and 128 characters.

The image map is uploaded with the Worker version. Updating the map does not restart or replace running Container instances. Your application selects an image from the map when it starts an instance. During a [gradual deployment](https://developers.cloudflare.com/workers/versions-and-deployments/gradual-deployments/), `ctx.container.images` reflects the image map of the Worker version that runs the Durable Object, so different Durable Objects can start different images until the deployment completes.

### Choose an instance size at runtime

Set `instance` in `ctx.container.start()` to one of the following [named instance types](https://developers.cloudflare.com/containers/platform/limits/#instance-types):

- `lite`
- `standard-1`
- `standard-2`
- `standard-3`
- `standard-4`

The runtime does not accept `basic` or the legacy `dev` and `standard` aliases.

If you omit `instance`, the Container uses `lite`. You can also supply a custom instance object that meets the [custom instance type constraints](https://developers.cloudflare.com/containers/platform/limits/#custom-instance-types):

*src/index.jsjs*

```js
this.ctx.container.start({
	image: this.ctx.container.images.base,
	enableInternet: false,
	instance: {
		vcpu: 1,
		memoryMib: 4096,
		diskMb: 8000,
	},
});
```

*src/index.tsts*

```ts
this.ctx.container.start({
	image: this.ctx.container.images.base,
	enableInternet: false,
	instance: {
		vcpu: 1,
		memoryMib: 4096,
		diskMb: 8000,
	},
});
```

The runtime uses camel case (`memoryMib` and `diskMb`). Wrangler's application-level `instance_type` object uses snake case (`memory_mib` and `disk_mb`) and only applies to the `default` policy.

### Start from an image or snapshot

For a new filesystem, pass `image` to `ctx.container.start()`. To restore a full filesystem snapshot, pass `containerSnapshot` instead. `image` and `containerSnapshot` are mutually exclusive because a snapshot already identifies the filesystem to restore.

Snapshots are only supported with the `durable_object` policy. A snapshot is tied to the image it was created from. After you update an image, create new snapshots from Containers running that image.

Refer to [Snapshots](https://developers.cloudflare.com/containers/guides/snapshots/) for the complete save and restore flow.

### Manage updates from application code

Container instances that use the `durable_object` policy do not participate in application-wide image rollouts. A running instance continues to use its startup image. Your application decides when to stop that instance and start it with another configured image. For an example that compares the running image with the configured image, refer to [Roll out a named image update](https://developers.cloudflare.com/containers/guides/image-management/#roll-out-a-named-image-update).

### Configure SSH

The `durable_object` policy supports the `ssh` and `authorized_keys` fields. Refer to [SSH](https://developers.cloudflare.com/containers/guides/ssh/).

## Related resources

- [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/)
- [Lifecycle of a Container](https://developers.cloudflare.com/containers/concepts/architecture/)
- [Image Management](https://developers.cloudflare.com/containers/guides/image-management/)
- [Wrangler Containers configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#containers)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/scheduling-policy/#page","headline":"Scheduling Policies","description":"Choose whether Container configuration is managed centrally or by each Durable Object at runtime.","url":"https://developers.cloudflare.com/containers/configuration/scheduling-policy/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/scheduling-policy/og.png?v=d30120184c1c529d","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Access KV, R2, Durable Objects, and other bindings from a container.
title: Connect to Workers and Bindings
image: https://developers.cloudflare.com/containers/configuration/workers-connections/og.png?v=157257af7f1fb147
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Connect to Workers and Bindings

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/workers-connections/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Containers can access [Workers bindings](https://developers.cloudflare.com/workers/runtime-apis/bindings/) — KV, R2, D1, Durable Objects, and others — through [outbound handlers](https://developers.cloudflare.com/containers/configuration/outbound-traffic/#define-outbound-handlers). An outbound handler intercepts HTTP requests from the container and runs inside the Workers runtime, where all of your configured bindings are available.

The container makes a plain HTTP request to a virtual hostname (for example, `http://my.kv/some-key`), and the outbound handler resolves it using the bound resource. No SDK or client library is required inside the container.

## Use bindings in outbound handlers

Define an `outboundByHost` handler for each virtual hostname. The `env` argument gives you access to every binding declared in your Wrangler configuration.

```js
export class MyContainer extends Container {}

MyContainer.outboundByHost = {
	"my.kv": async (request, env, ctx) => {
		const url = new URL(request.url);
		const key = url.pathname.slice(1);
		const value = await env.KV.get(key);
		return new Response(value);
	},
	"my.r2": async (request, env, ctx) => {
		const url = new URL(request.url);
		// Scope access to this container's ID
		const path = `${ctx.containerId}${url.pathname}`;
		const object = await env.R2.get(path);
		return new Response(object?.body ?? null, { status: object ? 200 : 404 });
	},
};
```

The container calls `http://my.kv/some-key` and the handler resolves it using the KV binding. A call to `http://my.r2/file.png` reads from R2, scoped to the current container instance.

Note

You can use `ctx.containerId` to apply different rules per container instance — for example, to look up per-instance configuration from KV.

## Access Durable Object state

The `ctx` argument exposes `containerId`, which lets you interact with the container's own Durable Object from an outbound handler.

```js
"get-state.do": async (request, env, ctx) => {
  const id = env.MY_CONTAINER.idFromString(ctx.containerId);
  const stub = env.MY_CONTAINER.get(id);
  // Assumes getStateForKey is defined on your DO
  return stub.getStateForKey(request.body);
},
```

## Related resources

- [Handle outbound traffic](https://developers.cloudflare.com/containers/configuration/outbound-traffic/) — Block, allow, and intercept outbound HTTP from a container
- [Environment variables and secrets](https://developers.cloudflare.com/containers/configuration/environment-variables/) — Configure secrets and environment variables
- [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/) — Full `ctx.container` API reference

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/workers-connections/#page","headline":"Connect to Workers and Bindings","description":"Access KV, R2, Durable Objects, and other bindings from a container.","url":"https://developers.cloudflare.com/containers/configuration/workers-connections/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/workers-connections/og.png?v=157257af7f1fb147","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Configure a Container, its Durable Object binding, and its class export in Wrangler.
title: Wrangler configuration
image: https://developers.cloudflare.com/containers/configuration/wrangler/og.png?v=2a187101b50ae69b
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Wrangler configuration

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/configuration/wrangler/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Define Containers in the Wrangler configuration file for your Worker. Each Container is associated with a Durable Object class, which provides access to the Container at runtime.

## Minimal configuration

A new Container application requires a Container definition, a Durable Object binding, and a Durable Object class export:

```jsonc
{
	"$schema": "./node_modules/wrangler/config-schema.json",
	"name": "my-container-worker",
	"main": "src/index.ts",
	"upload_source_maps": true,
	// Set this to today's date
	"compatibility_date": "2026-10-14",
	"containers": [
		{
			"class_name": "MyContainer",
			"image": "./Dockerfile",
			"max_instances": 10,
		},
	],
	"durable_objects": {
		"bindings": [
			{
				"name": "MY_CONTAINER",
				"class_name": "MyContainer",
			},
		],
	},
	"exports": {
		"MyContainer": {
			"type": "durable-object",
			"storage": "sqlite",
		},
	},
}
```

```toml
"$schema" = "./node_modules/wrangler/config-schema.json"
name = "my-container-worker"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[[containers]]
class_name = "MyContainer"
image = "./Dockerfile"
max_instances = 10

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"
```

The configuration uses three sections:

1. **`containers`** defines the container image and associates it with a Durable Object class through `class_name`.
2. **`durable_objects.bindings`** makes the Durable Object namespace available to Worker code. In this example, access it through `env.MY_CONTAINER`.
3. **`exports`** declares the Durable Object class and provisions it with SQLite storage.

The `class_name` in `containers` and `durable_objects.bindings`, and the key in `exports`, must match the exported Durable Object class in your Worker.

Existing applications

Existing applications that use the legacy `migrations` array can continue to use it. Do not configure `exports` and `migrations` together. To switch an existing application to `exports`, refer to [Migrate from the legacy `migrations` flow](https://developers.cloudflare.com/durable-objects/reference/durable-objects-migrations/#migrate-from-the-legacy-migrations-flow).

## Container settings

The `containers` entry can also configure the instance type, maximum number of running instances, image build, placement constraints, rollouts, and SSH access.

For all available fields and values, refer to the [Containers Wrangler configuration reference](https://developers.cloudflare.com/workers/wrangler/configuration/#containers).

## Next steps

- [Deploy Containers](https://developers.cloudflare.com/containers/guides/deploy/) — Build the image and deploy the Worker.
- [Scaling and Routing](https://developers.cloudflare.com/containers/configuration/scaling-and-routing/) — Route requests and scale Container instances.
- [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/) — Control how configuration changes reach running instances.
- [Image management](https://developers.cloudflare.com/containers/guides/image-management/) — Use local and remote container images.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/configuration/wrangler/#page","headline":"Wrangler configuration","description":"Configure a Container, its Durable Object binding, and its class export in Wrangler.","url":"https://developers.cloudflare.com/containers/configuration/wrangler/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/configuration/wrangler/og.png?v=2a187101b50ae69b","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Use the Durable Object Container API for new applications and compare it with the Container class.
title: API
image: https://developers.cloudflare.com/containers/api/og.png?v=b5325a11e1e8cc4d
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# API

Last updated Sep 29, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/api/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Containers provide two APIs for managing a container from a Durable Object. Both APIs address the same container runtime.

For new applications, we recommend the Durable Object Container API. It lets you combine container workloads with the Durable Object's persistent storage, alarms, and request handling. Use `ctx.container` to control the container lifecycle while your Durable Object coordinates application state. The `Container` class remains documented for existing applications.

### [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/)

Start, stop, monitor, and connect to a container through `ctx.container`.

### [Container class](https://developers.cloudflare.com/containers/api/container-class/)

Reference the class and its routing, readiness checks, lifecycle hooks, and scheduling for existing applications.

## Choose an API

Inside a Durable Object, use `ctx.container` to control the container runtime directly. You can manage startup, shutdown, networking, and resource usage while using Durable Object storage and alarms for state and coordination. Add readiness checks, custom request routing, or lifecycle policies when needed.

The `Container` class extends `DurableObject` and wraps the container runtime API with convenience methods. Existing applications can use these methods for request proxying, readiness checks, lifecycle hooks, and scheduling. Durable Object storage and alarms remain available when needed.

The following examples compare the starting structure for each API:

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {}

export class MyContainer extends DurableObject<Env> {
	fetch(): Response {
		const container = this.ctx.container;
		if (!container) {
			return new Response("No container is configured", { status: 500 });
		}
		if (container.running) {
			return new Response("Container is running");
		}
		container.start();
		return new Response("Container is starting", { status: 202 });
	}
}
```

*src/index.tsts*

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;
	sleepAfter = "10m";
}
```

Starting a container through the Durable Object Container API does not mean its ports are ready. For request routing, use [`getTcpPort()`](https://developers.cloudflare.com/containers/api/durable-object-container/#gettcpport) after checking port readiness. For all direct methods, refer to the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/). For convenience methods, refer to the [Container class API](https://developers.cloudflare.com/containers/api/container-class/).

The following table compares both options:

| Requirement | Durable Object Container API | `Container` class |
| --- | --- | --- |
| Start and stop a container | [`start()`](https://developers.cloudflare.com/containers/api/durable-object-container/#start), [`signal()`](https://developers.cloudflare.com/containers/api/durable-object-container/#signal), and [`destroy()`](https://developers.cloudflare.com/containers/api/durable-object-container/#destroy) | [`start()`](https://developers.cloudflare.com/containers/api/container-class/#start), [`stop()`](https://developers.cloudflare.com/containers/api/container-class/#stop), and [`destroy()`](https://developers.cloudflare.com/containers/api/container-class/#destroy) |
| Send and proxy traffic | [`getTcpPort(port).fetch()`](https://developers.cloudflare.com/containers/api/durable-object-container/#gettcpport) and [`getTcpPort(port).connect()`](https://developers.cloudflare.com/containers/api/durable-object-container/#gettcpport) | [`fetch()`](https://developers.cloudflare.com/containers/api/container-class/#fetch) and [`containerFetch()`](https://developers.cloudflare.com/containers/api/container-class/#containerfetch) |
| Execute another process | [`exec()`](https://developers.cloudflare.com/containers/api/durable-object-container/#exec) | [`ctx.container.exec()`](https://developers.cloudflare.com/containers/api/container-class/#execute-commands) |
| Check port readiness | Use [`getTcpPort()`](https://developers.cloudflare.com/containers/api/durable-object-container/#gettcpport) in application code | [`startAndWaitForPorts()`](https://developers.cloudflare.com/containers/api/container-class/#startandwaitforports) and [`waitForPort()`](https://developers.cloudflare.com/containers/api/container-class/#waitforport) |
| Handle concurrent starts | Coordinate calls to [`start()`](https://developers.cloudflare.com/containers/api/durable-object-container/#start) when needed | Handled by [`start()`](https://developers.cloudflare.com/containers/api/container-class/#start) and [`startAndWaitForPorts()`](https://developers.cloudflare.com/containers/api/container-class/#startandwaitforports) |
| Run lifecycle hooks | [`monitor()`](https://developers.cloudflare.com/containers/api/durable-object-container/#monitor) and application code | [`onStart()`](https://developers.cloudflare.com/containers/api/container-class/#onstart), [`onStop()`](https://developers.cloudflare.com/containers/api/container-class/#onstop), [`onError()`](https://developers.cloudflare.com/containers/api/container-class/#onerror), and [`onActivityExpired()`](https://developers.cloudflare.com/containers/api/container-class/#onactivityexpired) |
| Stop inactive containers | [`setInactivityTimeout()`](https://developers.cloudflare.com/containers/api/durable-object-container/#setinactivitytimeout) | [`sleepAfter`](https://developers.cloudflare.com/containers/api/container-class/#sleepafter) and [`onActivityExpired()`](https://developers.cloudflare.com/containers/api/container-class/#onactivityexpired) |
| Schedule callbacks | [`ctx.storage.setAlarm()`](https://developers.cloudflare.com/durable-objects/api/alarms/#setalarm) (Durable Object API) | [`schedule()`](https://developers.cloudflare.com/containers/api/container-class/#schedule) |

For new applications, use the Durable Object Container API to combine direct container control with Durable Object storage and coordination. It also supports latency-sensitive workloads and applications that need a smaller storage footprint.

If your application uses the `Container` class, refer to [Migrate to the Durable Object Container API](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/api/#page","headline":"API","description":"Use the Durable Object Container API for new applications and compare it with the Container class.","url":"https://developers.cloudflare.com/containers/api/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/api/og.png?v=b5325a11e1e8cc4d","dateModified":"2026-09-29","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: API reference for the Container class and its built-in lifecycle helpers.
title: Container class
image: https://developers.cloudflare.com/containers/api/container-class/og.png?v=26c6ffaa28be5042
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Container class

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/api/container-class/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

The [`Container` class ↗︎](https://github.com/cloudflare/containers) from [`@cloudflare/containers` ↗︎](https://www.npmjs.com/package/@cloudflare/containers) provides lifecycle helpers for container instances. For direct lifecycle control, use the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/).

**`Container` extends [`DurableObject`](https://developers.cloudflare.com/durable-objects/api/base/).** The Durable Object manages routing, persistent state, and lifecycle hooks, while the container process runs your image inside a Linux VM. Because your subclass is a Durable Object, you have access to the full Durable Object API — including [`this.ctx.storage`](https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/) for persistent SQLite-backed storage and [`this.ctx.id`](https://developers.cloudflare.com/durable-objects/api/id/) for the unique instance identifier. Use Durable Object storage to persist state that should survive container restarts, such as configuration, user data, or task results.

Note

The `Container` class does not support applications that use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy). It is available only with the `default` scheduling policy. For `durable_object` applications, use the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/).

To move an existing application to direct control, refer to [Migrate to the Durable Object Container API](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/).

## Use the Container class

Start by installing the `@cloudflare/containers` package:

npmyarnpnpmbun

```
npm i @cloudflare/containers
```

```
yarn add @cloudflare/containers
```

```
pnpm add @cloudflare/containers
```

```
bun add @cloudflare/containers
```

Then, define a class that extends `Container` and set the shared properties on the class:

```js
import { Container, getContainer } from "@cloudflare/containers";

export class SandboxContainer extends Container {
	defaultPort = 8080;
	requiredPorts = [8080, 9222];
	sleepAfter = "5m";
	envVars = {
		NODE_ENV: "production",
		LOG_LEVEL: "info",
	};
	entrypoint = ["npm", "run", "start"];
	enableInternet = false;
	pingEndpoint = "localhost/ready";
}

export default {
	async fetch(request, env) {
		return getContainer(env.SANDBOX_CONTAINER, "workspace-123").fetch(request);
	},
};
```

```plaintext
import { Container, getContainer } from "@cloudflare/containers";

export class SandboxContainer extends Container {
	defaultPort = 8080;
	requiredPorts = [8080, 9222];
	sleepAfter = "5m";
	envVars = {
		NODE_ENV: "production",
		LOG_LEVEL: "info",
	};
	entrypoint = ["npm", "run", "start"];
	enableInternet = false;
	pingEndpoint = "localhost/ready";
}

export default {
	async fetch(request: Request, env) {
		return getContainer(env.SANDBOX_CONTAINER, "workspace-123").fetch(request);
	},
};
```

The `Container` class extends `DurableObject`, so all [Durable Object](https://developers.cloudflare.com/durable-objects/) functionality is available — including [SQLite storage](https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/), [alarms](https://developers.cloudflare.com/durable-objects/api/alarms/), and [RPC methods](https://developers.cloudflare.com/durable-objects/api/base/#rpc-methods). Container disk is ephemeral by default, but Durable Object storage persists across container restarts.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async runAndPersist() {
		const res = await this.containerFetch("/run-task");
		const body = await res.text();
		this.ctx.storage.sql.exec(
			"INSERT OR REPLACE INTO results (value) VALUES (?)",
			body,
		);
		return body;
	}
}
```

```ts
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async runAndPersist() {
		const res = await this.containerFetch("/run-task");
		const body = await res.text();
		this.ctx.storage.sql.exec(
			"INSERT OR REPLACE INTO results (value) VALUES (?)",
			body,
		);
		return body;
	}
}
```

## Execute commands

Use `this.ctx.container.exec()` to start another process inside a running Container. Refer to [Execute commands](https://developers.cloudflare.com/containers/guides/execute-commands/) for startup, streaming, output, and process-control examples.

## Properties

Configure these as class fields on your subclass. They apply to every instance of the container.

- **`defaultPort`** ( `number`, optional) — the port your container process listens on. [`fetch()`](#fetch) and [`containerFetch()`](#containerfetch) forward requests here unless you specify a different port via [`switchPort()`](#switchport) or the `port` argument to [`containerFetch()`](#containerfetch). Most subclasses set this.
- **`requiredPorts`** ( `number[]`, optional) — ports that must be accepting connections before the container is considered ready. Used by [`startAndWaitForPorts()`](#startandwaitforports) when no `ports` argument is passed. Set this when your container runs multiple services that all need to be healthy before serving traffic.
- **`sleepAfter`** ( `string | number`, default: `"10m"`) — how long to keep the container alive without activity before shutting it down. Accepts a number of seconds or a duration string such as `"30s"`, `"5m"`, or `"1h"`. Activity resets the timer — see [`renewActivityTimeout()`](#renewactivitytimeout) for manual resets.
- **`envVars`** ( `Record<string, string>`, default: `{}`) — environment variables passed to the container on every start. For per-instance variables, pass `envVars` through [`startAndWaitForPorts()`](#startandwaitforports) instead.
- **`entrypoint`** ( `string[]`, optional) — overrides the image's default entrypoint. Useful when you want to run a different command without rebuilding the image, such as a dev server or a one-off task.
- **`enableInternet`** ( `boolean`, default: `true`) — controls whether the container can make outbound HTTP requests. Set to `false` for sandboxed environments where you want to intercept or block all outbound traffic. For more information, refer to [Handle outbound traffic](https://developers.cloudflare.com/containers/configuration/outbound-traffic/).
- **`pingEndpoint`** ( `string`, default: `"ping"`) — the host and path the class uses to health-check the container during startup. Most users do not need to change this.

## Lifecycle hooks

Override these methods to run Worker code when the container changes state. Refer to the [status hooks example](https://developers.cloudflare.com/containers/examples/status-hooks/) for a full example.

### `onStart`

Run Worker code after the container has started.

```ts
onStart(): void | Promise<void>
```

**Returns**: `void | Promise<void>`. Resolve after any startup logic finishes.

Use this to log startup, seed data, or schedule recurring tasks with [`schedule()`](#schedule).

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async onStart() {
		await this.containerFetch("http://localhost/bootstrap", {
			method: "POST",
		});
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    override async onStart() {
    	await this.containerFetch("http://localhost/bootstrap", {
    		method: "POST",
    	});
    }

}
```

### `onStop`

Run Worker code after the container process exits.

```ts
onStop(params: StopParams): void | Promise<void>
```

**Parameters**:

- `params.exitCode` - Container process exit code.
- `params.reason` - Why the container stopped: `'exit'` when the process exited on its own, or `'runtime_signal'` when the runtime signalled it.

**Returns**: `void | Promise<void>`. Resolve after your shutdown logic finishes.

Use this to log, alert, or restart the container.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	onStop({ exitCode, reason }) {
		console.log("Container stopped", { exitCode, reason });
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	override onStop({ exitCode, reason }) {
		console.log("Container stopped", { exitCode, reason });
	}
}
```

### `onError`

Handle startup and port-checking errors.

```ts
onError(error: unknown): any
```

**Parameters**:

- `error` - The error thrown during startup or port checks.

**Returns**: `any`. The default implementation logs the error and re-throws it.

Override this to suppress errors, notify an external service, or attempt a restart.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	onError(error) {
		console.error("Container failed to start", error);
		throw error;
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	override onError(error: unknown) {
		console.error("Container failed to start", error);
		throw error;
	}
}
```

### `onActivityExpired`

Run Worker code when the [`sleepAfter`](#sleepafter) timer expires.

```ts
onActivityExpired(): Promise<void>
```

**Returns**: `Promise<void>`. Resolve after your idle-time logic finishes.

Called when the [`sleepAfter`](#sleepafter) timeout expires with no incoming requests. The default implementation calls [`stop()`](#stop).

Caution

If you override `onActivityExpired()`, call [`await this.stop()`](#stop) or [`await this.destroy()`](#destroy). Otherwise, the container does not go to sleep.

If you override this method without stopping the container, the timer renews and the hook fires again on the next expiry.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	sleepAfter = "2m";

	async onActivityExpired() {
		const state = await this.getState();
		console.log("Container is idle, stopping it now", state.status);

		await this.stop();
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	sleepAfter = "2m";

    override async onActivityExpired() {
    	const state = await this.getState();
    	console.log("Container is idle, stopping it now", state.status);

    	await this.stop();
    }

}
```

## Request methods

### `fetch`

Handle incoming HTTP or WebSocket requests.

```ts
fetch(request: Request): Promise<Response>
```

**Parameters**:

- `request` - The incoming request to proxy to the container.

**Returns**: `Promise<Response>` from the container or from your custom routing logic.

By default, `fetch` forwards the request to the container process at [`defaultPort`](#defaultport). The container is started automatically if it is not already running.

Override `fetch` when you need routing logic, authentication, or other middleware before forwarding to the container. Inside the override, call [`this.containerFetch()`](#containerfetch) rather than `this.fetch()` to avoid infinite recursion:

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async fetch(request) {
		const url = new URL(request.url);

		if (url.pathname === "/health") {
			return new Response("ok");
		}

		return this.containerFetch(request);
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    override async fetch(request: Request): Promise<Response> {
    	const url = new URL(request.url);

    	if (url.pathname === "/health") {
    		return new Response("ok");
    	}

    	return this.containerFetch(request);
    }

}
```

`fetch` is the only method that supports WebSocket proxying. Refer to the [WebSocket example](https://developers.cloudflare.com/containers/examples/websocket/) for a full example.

### `containerFetch`

Send an HTTP request directly to the container process. Generally, users should prefer to use [`fetch`](#fetch) unless it has been overridden.

```ts
containerFetch(request: Request, port?: number): Promise<Response>
containerFetch(url: string | URL, init?: RequestInit, port?: number): Promise<Response>
```

**Parameters**:

- `request` - Existing `Request` object to forward.
- `url` - URL to request when you are constructing a new request.
- `init` - Standard `RequestInit` options for the URL-based overload.
- `port` - Optional target port. If omitted, the class uses [`defaultPort`](#defaultport).

**Returns**: `Promise<Response>` from the container.

This is what the default [`fetch()`](#fetch) implementation calls internally, and it is what you should call from within an overridden [`fetch()`](#fetch) method to avoid infinite recursion. It also accepts a standard fetch-style signature with a URL string and `RequestInit`, which is useful when you are constructing a new request rather than forwarding an existing one.

Does not support WebSockets. Use [`fetch()`](#fetch) with [`switchPort()`](#switchport) for those.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async fetch(request) {
		const url = new URL(request.url);

		if (url.pathname === "/metrics") {
			return this.containerFetch(
				"http://localhost/internal/metrics",
				{
					headers: {
						authorization: request.headers.get("authorization") ?? "",
					},
				},
				9090,
			);
		}

		return this.containerFetch(request);
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    override async fetch(request: Request): Promise<Response> {
    	const url = new URL(request.url);

    	if (url.pathname === "/metrics") {
    		return this.containerFetch(
    			"http://localhost/internal/metrics",
    			{
    				headers: {
    					authorization: request.headers.get("authorization") ?? "",
    				},
    			},
    			9090,
    		);
    	}

    	return this.containerFetch(request);
    }

}
```

## Start and stop

In most cases you do not need to call these methods directly. [`fetch()`](#fetch) and [`containerFetch()`](#containerfetch) start the container automatically. Call these explicitly when you need to pre-warm a container, run a task on a schedule, or control the lifecycle from within a lifecycle hook.

### `startAndWaitForPorts`

Start the container and wait until the target ports are accepting connections.

```ts
startAndWaitForPorts(args?: StartAndWaitForPortsOptions): Promise<void>
startAndWaitForPorts(
  ports?: number | number[],
  cancellationOptions?: CancellationOptions,
  startOptions?: ContainerStartConfigOptions,
): Promise<void>
```

**Parameters**:

- `args.ports` - Port or ports to wait for. Port resolution order is explicit `ports`, then [`requiredPorts`](#requiredports), then [`defaultPort`](#defaultport).
- `args.startOptions` - Per-instance startup overrides.
- `args.startOptions.envVars` - Per-instance environment variables.
- `args.startOptions.entrypoint` - Entrypoint override for this start only.
- `args.startOptions.enableInternet` - Whether outbound internet access is allowed for this start.
- `args.cancellationOptions.abort` - Abort signal to cancel startup.
- `args.cancellationOptions.instanceGetTimeoutMS` - Maximum time to get a container instance and issue the start command. Default: `8000`.
- `args.cancellationOptions.portReadyTimeoutMS` - Maximum time to wait for all ports to become ready. Default: `20000`.
- `args.cancellationOptions.waitInterval` - Polling interval in milliseconds. Default: `300`.

**Returns**: `Promise<void>`. Resolves after the target ports are ready and [`onStart()`](#onstart) has run.

This is the safest way to explicitly start a container when you need to be certain it is ready before sending traffic.

This method also supports positional `ports`, `cancellationOptions`, and `startOptions` arguments, but the object form is easier to read.

```js
import { getContainer } from "@cloudflare/containers";

export default {
	async scheduled(_event, env) {
		const container = getContainer(env.API_CONTAINER, "tenant-42");

		await container.startAndWaitForPorts({
			ports: [8080, 9222],
			startOptions: {
				envVars: {
					API_KEY: env.API_KEY,
					TENANT_ID: "tenant-42",
				},
			},
			cancellationOptions: {
				portReadyTimeoutMS: 30_000,
			},
		});
	},
};
```

```plaintext
import { getContainer } from "@cloudflare/containers";

export default {
	async scheduled(_event, env) {
		const container = getContainer(env.API_CONTAINER, "tenant-42");

    	await container.startAndWaitForPorts({
    		ports: [8080, 9222],
    		startOptions: {
    			envVars: {
    				API_KEY: env.API_KEY,
    				TENANT_ID: "tenant-42",
    			},
    		},
    		cancellationOptions: {
    			portReadyTimeoutMS: 30_000,
    		},
    	});
    },

};
```

Refer to the [env vars and secrets example](https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/) for a full example.

### `start`

Start the container without waiting for all ports to become ready.

```ts
start(startOptions?: ContainerStartConfigOptions, waitOptions?: WaitOptions): Promise<void>
```

**Parameters**:

- `startOptions` - Per-instance startup overrides.
- `startOptions.envVars` - Per-instance environment variables.
- `startOptions.entrypoint` - Entrypoint override for this start only.
- `startOptions.enableInternet` - Whether outbound internet access is allowed for this start.
- `waitOptions.portToCheck` - Port to probe while starting. If omitted, the class uses [`defaultPort`](#defaultport), the first [`requiredPorts`](#requiredports) entry, or a fallback port.
- `waitOptions.signal` - Abort signal to cancel startup.
- `waitOptions.retries` - Maximum number of start attempts before the method throws.
- `waitOptions.waitInterval` - Polling interval in milliseconds between retries.

**Returns**: `Promise<void>`. Resolves after the start attempt succeeds and [`onStart()`](#onstart) has run.

Use this when the container does not expose ports, such as a batch job or a cron task, or when you want to manage readiness yourself with [`waitForPort()`](#waitforport). If you need to wait for all ports to be ready, use [`startAndWaitForPorts()`](#startandwaitforports) instead.

```js
import { getContainer } from "@cloudflare/containers";

export default {
	async scheduled(_event, env) {
		const container = getContainer(env.JOB_CONTAINER, "nightly-report");

		await container.start({
			entrypoint: ["node", "scripts/nightly-report.js"],
			envVars: {
				REPORT_DATE: new Date().toISOString(),
			},
			enableInternet: false,
		});
	},
};
```

```plaintext
import { getContainer } from "@cloudflare/containers";

export default {
	async scheduled(_event, env) {
		const container = getContainer(env.JOB_CONTAINER, "nightly-report");

    	await container.start({
    		entrypoint: ["node", "scripts/nightly-report.js"],
    		envVars: {
    			REPORT_DATE: new Date().toISOString(),
    		},
    		enableInternet: false,
    	});
    },

};
```

Refer to the [cron example](https://developers.cloudflare.com/containers/examples/cron/) for a full example.

### `waitForPort`

Poll a single port until it accepts connections.

```ts
waitForPort(waitOptions: WaitOptions): Promise<number>
```

**Parameters**:

- `waitOptions.portToCheck` - Port number to check.
- `waitOptions.signal` - Abort signal to cancel waiting.
- `waitOptions.retries` - Maximum number of retries before the method throws.
- `waitOptions.waitInterval` - Polling interval in milliseconds.

**Returns**: `Promise<number>`. The numeric return value is mainly useful when you are coordinating custom readiness logic across multiple waits.

Throws if the port does not become available within the retry limit. Use this after [`start()`](#start) when you need to check multiple ports independently or in a specific sequence.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async warmInspector() {
		await this.start();

		const retryCount = await this.waitForPort({
			portToCheck: 9222,
			retries: 20,
			waitInterval: 500,
		});

		console.log("Inspector port became ready:", retryCount);
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async warmInspector() {
		await this.start();

    	const retryCount = await this.waitForPort({
    		portToCheck: 9222,
    		retries: 20,
    		waitInterval: 500,
    	});

    	console.log("Inspector port became ready:", retryCount);
    }

}
```

### `stop`

Send a signal to the container process.

```ts
stop(signal?: 'SIGTERM' | 'SIGINT' | 'SIGKILL' | number): Promise<void>
```

**Parameters**:

- `signal` - Signal to send. Defaults to `'SIGTERM'`.

**Returns**: `Promise<void>`. Resolves after the signal is sent and pending stop handling has completed.

Defaults to `SIGTERM`, which gives the process a chance to shut down gracefully. Triggers [`onStop()`](#onstop).

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async fetch(request) {
		if (new URL(request.url).pathname === "/admin/stop") {
			await this.stop();
			return new Response("Container is stopping");
		}

		return this.containerFetch(request);
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    override async fetch(request: Request): Promise<Response> {
    	if (new URL(request.url).pathname === "/admin/stop") {
    		await this.stop();
    		return new Response("Container is stopping");
    	}

    	return this.containerFetch(request);
    }

}
```

### `destroy`

Immediately kill the container process.

```ts
destroy(): Promise<void>
```

**Returns**: `Promise<void>`. Resolves after the runtime has destroyed the container.

This sends `SIGKILL`. Use it when you need the container gone immediately and cannot wait for a graceful shutdown. Triggers [`onStop()`](#onstop).

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async fetch(request) {
		if (new URL(request.url).pathname === "/admin/destroy") {
			await this.destroy();
			return new Response("Container destroyed");
		}

		return this.containerFetch(request);
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    override async fetch(request: Request): Promise<Response> {
    	if (new URL(request.url).pathname === "/admin/destroy") {
    		await this.destroy();
    		return new Response("Container destroyed");
    	}

    	return this.containerFetch(request);
    }

}
```

## State and monitoring

### `getState`

Read the current container state.

```ts
getState(): Promise<State>
```

**Returns**: `Promise<State>` with:

- `status` - One of `'running'`, `'healthy'`, `'stopping'`, `'stopped'`, or `'stopped_with_code'`.
- `lastChange` - Unix timestamp in milliseconds for the last state change.
- `exitCode` - Optional exit code when `status` is `'stopped_with_code'`.

`running` means the container is starting and has not yet passed its health check. `healthy` means it is up and accepting requests.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async logState() {
		const state = await this.getState();

		if (state.status === "stopped_with_code") {
			console.error("Container exited with code", state.exitCode);
			return;
		}

		console.log("Container status:", state.status);
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	async logState() {
		const state = await this.getState();

    	if (state.status === "stopped_with_code") {
    		console.error("Container exited with code", state.exitCode);
    		return;
    	}

    	console.log("Container status:", state.status);
    }

}
```

### `renewActivityTimeout`

Reset the [`sleepAfter`](#sleepafter) timer.

```ts
renewActivityTimeout(): void
```

**Returns**: `void`.

Incoming requests reset the timer automatically. Call this manually from background work, such as a scheduled task or a long-running operation, that should count as activity and prevent the container from sleeping.

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async processJobs(jobIds) {
		for (const jobId of jobIds) {
			this.renewActivityTimeout();

			await this.containerFetch(`http://localhost/jobs/${jobId}`, {
				method: "POST",
			});
		}
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    async processJobs(jobIds: string[]) {
    	for (const jobId of jobIds) {
    		this.renewActivityTimeout();

    		await this.containerFetch(`http://localhost/jobs/${jobId}`, {
    			method: "POST",
    		});
    	}
    }

}
```

## Scheduling

### `schedule`

Schedule a method on the class to run later.

```ts
schedule<T>(when: Date | number, callback: string, payload?: T): Promise<Schedule<T>>
```

**Parameters**:

- `when` - Either a `Date` for a specific time or a number of seconds to delay.
- `callback` - Name of the class method to call.
- `payload` - Optional data passed to the callback method.

**Returns**: `Promise<Schedule<T>>` with:

- `taskId` - Unique schedule ID.
- `callback` - Method name that will be called.
- `payload` - Payload that will be passed to the callback.
- `type` - `'scheduled'` for an absolute time or `'delayed'` for a relative delay.
- `time` - Unix timestamp in seconds when the task will run.
- `delayInSeconds` - Delay in seconds when `type` is `'delayed'`.

Do not override [`alarm()` ↗︎](https://developers.cloudflare.com/durable-objects/api/alarms/) directly. The `Container` class uses the alarm handler to manage the container lifecycle, so use [`schedule()`](#schedule) instead.

The following example schedules a recurring health report starting at container startup:

```js
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

	async onStart() {
		await this.schedule(60, "healthReport");
	}

	async healthReport() {
		const state = await this.getState();
		console.log("Container status:", state.status);
		await this.schedule(60, "healthReport");
	}
}
```

```plaintext
import { Container } from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;

    override async onStart() {
    	await this.schedule(60, "healthReport");
    }

    async healthReport() {
    	const state = await this.getState();
    	console.log("Container status:", state.status);
    	await this.schedule(60, "healthReport");
    }

}
```

## Outbound interception

Outbound interception lets you intercept, mock, or block HTTP requests that the container makes to external hosts. This is useful for sandboxing, testing, or proxying outbound traffic through Worker code.

```js
import {
	Container,
	ContainerProxy,
	getContainer,
} from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;
	enableInternet = true;

	static outboundByHost = {
		"blocked.example.com": () => {
			return new Response("Blocked", { status: 403 });
		},
	};

	static outbound = async (request, _env, ctx) => {
		console.log(`[${ctx.containerId}] outbound:`, request.url);
		return fetch(request);
	};
}

export { ContainerProxy };

export default {
	async fetch(request, env) {
		return getContainer(env.MY_CONTAINER).fetch(request);
	},
};
```

```plaintext
import {
	Container,
	ContainerProxy,
	getContainer,
} from "@cloudflare/containers";

export class MyContainer extends Container {
	defaultPort = 8080;
	enableInternet = true;

    static outboundByHost = {
    	"blocked.example.com": () => {
    		return new Response("Blocked", { status: 403 });
    	},
    };

    static outbound = async (request, _env, ctx) => {
    	console.log(`[${ctx.containerId}] outbound:`, request.url);
    	return fetch(request);
    };

}

export { ContainerProxy };

export default {
	async fetch(request: Request, env) {
		return getContainer(env.MY_CONTAINER).fetch(request);
	},
};
```

For more information, refer to [Handle outbound traffic](https://developers.cloudflare.com/containers/configuration/outbound-traffic/).

## Utility functions

These functions are exported alongside the `Container` class from `@cloudflare/containers`.

### `getContainer`

Get a stub for a named container instance.

```ts
getContainer<T>(binding: DurableObjectNamespace<T>, name?: string): DurableObjectStub<T>
```

**Parameters**:

- `binding` - Durable Object namespace binding for your container class.
- `name` - Stable instance name. Defaults to `cf-singleton-container`.

**Returns**: `DurableObjectStub<T>` for the named container instance.

Use this when you want one container per logical entity, such as a user session, a document, or a game room, identified by a stable name.

```js
import { getContainer } from "@cloudflare/containers";

export default {
	async fetch(request, env) {
		const { sessionId } = await request.json();
		return getContainer(env.MY_CONTAINER, sessionId).fetch(request);
	},
};
```

```plaintext
import { getContainer } from "@cloudflare/containers";

export default {
	async fetch(request: Request, env) {
		const { sessionId } = await request.json();
		return getContainer(env.MY_CONTAINER, sessionId).fetch(request);
	},
};
```

### `getRandom`

Get a stub for a randomly selected container instance.

```ts
getRandom<T>(binding: DurableObjectNamespace<T>, instances?: number): Promise<DurableObjectStub<T>>
```

**Parameters**:

- `binding` - Durable Object namespace binding for your container class.
- `instances` - Total number of instances to choose from. Defaults to `3`.

**Returns**: `Promise<DurableObjectStub<T>>` for the randomly selected instance.

Use this for stateless workloads where any container can handle any request and you want to spread load across multiple instances.

```js
import { getRandom } from "@cloudflare/containers";

export default {
	async fetch(request, env) {
		const container = await getRandom(env.WORKER_POOL, 5);
		return container.fetch(request);
	},
};
```

```plaintext
import { getRandom } from "@cloudflare/containers";

export default {
	async fetch(request: Request, env) {
		const container = await getRandom(env.WORKER_POOL, 5);
		return container.fetch(request);
	},
};
```

Refer to the [stateless instances example](https://developers.cloudflare.com/containers/examples/stateless/) for a full example.

### `switchPort`

Target a different container port while still using `fetch()`.

```ts
switchPort(request: Request, port: number): Request
```

**Parameters**:

- `request` - Request to copy.
- `port` - Port to encode into the request headers.

**Returns**: `Request` copy with the target port set.

Use this when you need to target a specific port and also need WebSocket support. If you do not need WebSockets, pass the port directly to [`containerFetch()`](#containerfetch) instead.

```js
import { getContainer, switchPort } from "@cloudflare/containers";

export default {
	async fetch(request, env) {
		const container = getContainer(env.MY_CONTAINER);
		return container.fetch(switchPort(request, 9090));
	},
};
```

```plaintext
import { getContainer, switchPort } from "@cloudflare/containers";

export default {
	async fetch(request: Request, env) {
		const container = getContainer(env.MY_CONTAINER);
		return container.fetch(switchPort(request, 9090));
	},
};
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/api/container-class/#page","headline":"Container class","description":"API reference for the Container class and its built-in lifecycle helpers.","url":"https://developers.cloudflare.com/containers/api/container-class/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/api/container-class/og.png?v=26c6ffaa28be5042","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Access and manage containers associated with a Durable Object, including start, stop, and interaction methods.
title: Durable Object Container API
image: https://developers.cloudflare.com/containers/api/durable-object-container/og.png?v=f13dbe1f95442c69
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Durable Object Container API

Last updated Sep 30, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/api/durable-object-container/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Each [container](https://developers.cloudflare.com/containers/) is managed and proxied by a Durable Object. The Durable Object manages routing and persistent state. The container process runs your image inside a Linux VM.

The API documented on this page is available on `this.ctx.container` inside any Durable Object class that has a container binding. Use it for direct control over the container process.

Note

We recommend starting new applications with the Durable Object Container API. It lets you run container workloads alongside the Durable Object's persistent storage, alarms, and request handling. Use `ctx.container` to control when the container starts, receives traffic, and stops, while the Durable Object manages application state and coordination. For existing applications that use the `Container` class, refer to [Migrate to the Durable Object Container API](https://developers.cloudflare.com/containers/guides/migrate-to-durable-object-container-api/).

Your Durable Object also has access to [SQLite storage](https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/) through `this.ctx.storage`, [alarms](https://developers.cloudflare.com/durable-objects/api/alarms/), and all other Durable Object APIs. Use storage to preserve configuration and state across container restarts. Use alarms to schedule work without keeping the container running.

*index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
	async start() {
		const container = this.ctx.container;
		if (!container) {
			throw new Error("No container is configured for this Durable Object");
		}
		if (!container.running) {
			// With the `default` scheduling policy, call `container.start()` without options.
			container.start({
				image: container.images.base,
				enableInternet: false,
			});
			await this.ctx.storage.put("lastStartedAt", Date.now());
		}
	}

	async scheduleStart(timestamp) {
		await this.ctx.storage.setAlarm(timestamp);
	}

	async alarm() {
		await this.start();
	}
}
```

*index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {}

export class MyDurableObject extends DurableObject<Env> {
	async start(): Promise<void> {
		const container = this.ctx.container;
		if (!container) {
			throw new Error("No container is configured for this Durable Object");
		}
		if (!container.running) {
			// With the `default` scheduling policy, call `container.start()` without options.
			container.start({
				image: container.images.base,
				enableInternet: false,
			});
			await this.ctx.storage.put("lastStartedAt", Date.now());
		}
	}

	async scheduleStart(timestamp: number): Promise<void> {
		await this.ctx.storage.setAlarm(timestamp);
	}

	async alarm(): Promise<void> {
		await this.start();
	}
}
```

## Attributes

### `images` durable\_object policy only

`images` is a read-only map generated from the [`images` field in the application's Wrangler configuration](https://developers.cloudflare.com/workers/wrangler/configuration/#durable_object-scheduling-policy). Wrangler builds or resolves each named image and exposes its digest-pinned reference under the same key. This attribute is available only for applications that use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy).

```js
const image = this.ctx.container.images.base;
```

Pass a value from this map as the `image` option to [`start()`](#start).

### `running`

`running` is `true` when the container is running. It does not confirm that the container is ready to accept requests.

```js
this.ctx.container.running;
```

## Methods

### `start`

`start()` boots a container. It returns before the container is ready to accept requests. Confirm readiness before sending traffic.

`start()` returns once it validates its options. To catch later errors, including a container that fails to start, use [`monitor()`](#monitor).

The required options depend on the application's scheduling policy.

```js
// `default` scheduling policy:
// The image and instance size come from Wrangler configuration.
this.ctx.container.start();

// `durable_object` scheduling policy:
// Pass an image and choose whether to allow outbound Internet access.
// The instance size is optional and defaults to "lite".
this.ctx.container.start({
	image: this.ctx.container.images.base,
	enableInternet: true,
});
```

#### Parameters

- `options` ( `object`, conditionally required): Startup options. Required for applications that use the `durable_object` scheduling policy and optional for applications that use the `default` scheduling policy.
  - `env` ( `Record<string, string>`, optional): Environment variables to pass to the container. Processes started with [`exec()`](#exec) do not receive these variables, except `PATH`.
  - `entrypoint` ( `string[]`, optional): Command and arguments to run in the container.
  - `enableInternet` ( `boolean`, required): Whether to allow outbound Internet access. Required when you pass `options`.
  - `image` ( `string`, conditionally required) durable\_object policy only: Image reference to start. Required unless you pass `containerSnapshot`. Pass the [`cloudflare/debian-trixie` managed image](https://developers.cloudflare.com/containers/guides/image-management/#use-the-cloudflare-managed-image) or a value from [`ctx.container.images`](#images).
  - `instance` ( `string | object`, optional) durable\_object policy only: Instance size. Pass `"lite"`, `"standard-1"`, `"standard-2"`, `"standard-3"`, `"standard-4"`, or a custom object with `vcpu`, `memoryMib`, and `diskMb` properties. `memoryMib` is in mebibytes and `diskMb` is in megabytes. Both must be positive integers. Defaults to `"lite"`.
  - `containerSnapshot` ( `ContainerSnapshotRestoreParams`, optional) durable\_object policy only: Snapshot handle to restore before startup. Pass the `ContainerSnapshot` returned by [`snapshotContainer()`](#snapshotcontainer), or an object containing its `id`. You cannot pass both `containerSnapshot` and `image`.
  - `labels` ( `Record<string, string>`, optional): Up to 10 labels that [`inspect()`](#inspect) returns. Label names must contain 1 to 16 bytes. Label values can contain up to 64 bytes. Names and values cannot contain control characters.

`monitor()` rejects when a container that uses the `durable_object` scheduling policy starts without `image` or `containerSnapshot`.

#### Return values

- `void`: No return value.

#### Exceptions

- `start()` throws an `Error` when the container is already running.
- `start()` throws a `TypeError` when both `image` and `containerSnapshot` are set, when either is empty, or when `instance` names an unknown type.
- `start()` throws a `RangeError` when a custom `instance` value is missing or invalid.
- `start()` throws an `Error` when `labels` exceed the label limits, when an environment variable name contains `=`, or when an environment variable name or value contains a null character.

### `inspect`

`inspect()` returns the image and labels for a running container. It returns `null` when no container is running.

```js
const containerInfo = await this.ctx.container.inspect();
```

#### Parameters

- None.

#### Return values

- `Promise<ContainerInfo | null>`: Resolves with `null` when no container is running, including after a failed start or after `destroy()`. Otherwise, it resolves with a `ContainerInfo` object containing:
  - `image` ( `string`): Image reference passed to [`start()`](#start). The value is an empty string while the container is starting and for a container restored from `containerSnapshot`.
  - `labels` ( `Record<string, string>`): Labels passed to [`start()`](#start).

### `exec`

`exec()` starts another process inside an already-running container. It does not start a stopped container. It waits for a container that is still starting.

```txt
exec(
  cmd: string[],
  options?: ContainerExecOptions,
): Promise<ExecProcess>
```

`exec()` starts the executable directly with the provided arguments. It does not start a shell or interpret pipes, redirects, expansion, or other shell syntax. Invoke Bash explicitly with `["bash", "-lc", "<COMMAND>"]` when Bash exists in the image. Use `["sh", "-c", "<COMMAND>"]` for images with only a Portable Operating System Interface (POSIX) shell.

The following RPC method checks that the container is running before executing a command:

```js
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
	async runCommand() {
		const container = this.ctx.container;
		if (!container) {
			throw new Error("No container is configured for this Durable Object");
		}
		if (!container.running) {
			throw new Error("Container is not running");
		}

		const process = await container.exec(["node", "--version"]);
		const output = await process.output();

		return {
			pid: process.pid,
			exitCode: output.exitCode,
			stdout: new TextDecoder().decode(output.stdout),
		};
	}
}
```

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {}

export class MyDurableObject extends DurableObject<Env> {
	async runCommand() {
		const container = this.ctx.container;
		if (!container) {
			throw new Error("No container is configured for this Durable Object");
		}
		if (!container.running) {
			throw new Error("Container is not running");
		}

		const process = await container.exec(["node", "--version"]);
		const output = await process.output();

		return {
			pid: process.pid,
			exitCode: output.exitCode,
			stdout: new TextDecoder().decode(output.stdout),
		};
	}
}
```

#### Parameters

- `cmd` ( `string[]`): Executable followed by its arguments.
- `options` ( `ContainerExecOptions`, optional): Process configuration:
  - `stdin` ( `ReadableStream | "pipe"`, optional): Source for standard input. Use `"pipe"` to write through the returned `stdin` stream. When omitted, standard input closes and sends end-of-file (EOF).
  - `stdout` ( `"pipe" | "ignore"`, optional, default `"pipe"`): Captures or discards standard output.
  - `stderr` ( `"pipe" | "ignore" | "combined"`, optional, default `"pipe"`): Captures, discards, or merges standard error into standard output. The `"combined"` value requires `stdout: "pipe"`. Combined output does not guarantee ordering between its source streams.
  - `cwd` ( `string`, optional): Working directory for the process.
  - `env` ( `Record<string, string>`, optional): Environment variables for the process. Other variables from `start()` or the image are not inherited, except for `PATH`. Unless overridden here, `PATH` uses the container's startup value, including any override passed to `start()`.
  - `user` ( `string`, optional): Numeric Linux user and group IDs for the process, as `uid:gid`. When omitted, the process runs as the image user.
  - `signal` ( `AbortSignal`, optional): Aborting the signal sends `SIGKILL` to the process. An already-aborted signal makes `exec()` throw an `AbortError`.
  - `pty` ( `boolean | { cols?: number; rows?: number }`, optional): Runs the process attached to a pseudo-terminal. `true` uses 80 columns and 24 rows.

The following call runs a command as user `1000` and group `1000`. The file it creates belongs to `1000:1000`:

```ts
const process = await this.ctx.container.exec(["touch", "/tmp/notes.txt"], {
	user: "1000:1000",
});
```

Caution

Pass both IDs in `user`. A user ID without a group ID runs the process as `root`, and a user or group name makes `exec()` reject with an internal error.

`user` does not restrict a process in a deployed container with the `durable_object` scheduling policy. Every process has the same Linux capabilities as `root`, regardless of user. File permissions do not stop the process, and it can switch back to `root`. The IDs set the owner of files that the process creates. In containers with the [`default` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/), other users have no capabilities, and file permissions apply.

With `pty`, `stdin` defaults to `"pipe"`, and `stderr` defaults to `"combined"` and cannot take another value. Standard output and standard error arrive together on `stdout`. The terminal converts line endings to `\r\n`.

#### Return values

- `Promise<ExecProcess>`: Resolves with an `ExecProcess` for the started process.

An `ExecProcess` has these fields and methods:

- `stdin` ( `WritableStream | undefined`): Writable standard input when `stdin` is `"pipe"`.
- `stdout` ( `ReadableStream | undefined`): Readable standard output when piped.
- `stderr` ( `ReadableStream | undefined`): Readable standard error when piped separately.
- `pid` ( `number`): Process identifier.
- `isPty` ( `boolean`): Whether the process runs attached to a pseudo-terminal.
- `exitCode` ( `Promise<number>`): Resolves when the process exits. Nonzero codes resolve normally instead of rejecting.
- `output()` ( `Promise<ExecOutput>`): Reads buffered output once. `ExecOutput` contains `stdout` ( `ArrayBuffer`), `stderr` ( `ArrayBuffer`), and `exitCode` ( `number`). Ignored streams produce empty buffers. Use `TextDecoder` to decode text.
- `kill(signal?: number)` ( `void`): Queues a signal for the process. The default is `SIGTERM`, signal `15`. The signal must be from `1` through `64`.
- `resize(cols: number, rows: number)` ( `void`): Changes the pseudo-terminal size. Both values must be from `1` through `65535`.

A stream that the process does not provide is `undefined`. For example, with `stderr: "combined"` or `pty`, `stderr` is `undefined` on `ExecProcess` and an empty `ArrayBuffer` on `ExecOutput`. Read both output channels from `stdout`.

The Workers TypeScript types declare `stdin`, `stdout`, and `stderr` as `| null`. Test for a stream, such as `if (process.stderr)`, instead of comparing with `null`.

`output()` throws a `TypeError` when called more than once or after either readable stream starts being consumed.

`output()` holds all of the output in the memory of the Durable Object, where it counts toward the [memory limit](https://developers.cloudflare.com/workers/platform/limits/#memory). When output can be large, read both streams concurrently instead. For more information, refer to [Stream large output](https://developers.cloudflare.com/containers/guides/execute-commands/#stream-large-output).

#### Process lifetime

`exec()` has no built-in timeout. Use `kill()` to request termination, then observe completion through `exitCode`. A process can handle or ignore a signal, so this does not enforce a hard deadline. Do not infer a specific exit code from the signal.

`kill()` and `signal` reach only the process that `exec()` started. Processes that this process starts, such as the commands in a `bash -c` script, keep running. `output()` waits until they exit. To stop them too, refer to [Stop the processes a command starts](https://developers.cloudflare.com/containers/guides/execute-commands/#stop-the-processes-a-command-starts).

Canceling the request that started a process does not stop the process. If the client retries, a second copy of the command runs.

To stop the process when the client disconnects, turn on the [`enable_request_signal`](https://developers.cloudflare.com/workers/configuration/compatibility-flags/#enable-requestsignal-for-incoming-requests) compatibility flag in the Worker that defines the Durable Object, and call the Durable Object through `fetch()` on its stub. Then abort the `exec()` signal when `request.signal` aborts. RPC methods do not receive a request signal.

```js
export class MyContainer extends DurableObject {
	async fetch(request) {
		const container = this.ctx.container;
		if (!container?.running) {
			return new Response("The container is not running", { status: 503 });
		}

		const controller = new AbortController();
		const abort = () => controller.abort();
		if (request.signal.aborted) {
			abort();
		} else {
			request.signal.addEventListener("abort", abort, { once: true });
		}

		const process = await container.exec(["sleep", "60"], {
			signal: controller.signal,
		});
		// Stop forwarding the abort once the process exits.
		void process.exitCode.then(() =>
			request.signal.removeEventListener("abort", abort),
		);

		const output = await process.output();
		return new Response(`Exit code: ${output.exitCode}`);
	}
}
```

```ts
export class MyContainer extends DurableObject {
	async fetch(request: Request) {
		const container = this.ctx.container;
		if (!container?.running) {
			return new Response("The container is not running", { status: 503 });
		}

		const controller = new AbortController();
		const abort = () => controller.abort();
		if (request.signal.aborted) {
			abort();
		} else {
			request.signal.addEventListener("abort", abort, { once: true });
		}

		const process = await container.exec(["sleep", "60"], {
			signal: controller.signal,
		});
		// Stop forwarding the abort once the process exits.
		void process.exitCode.then(() =>
			request.signal.removeEventListener("abort", abort),
		);

		const output = await process.output();
		return new Response(`Exit code: ${output.exitCode}`);
	}
}
```

Caution

Signal only a process that is still running. Aborting `signal` or calling `kill()` after the process exits raises an uncaught `internal error` in the Durable Object.

Do not pass `AbortSignal.timeout()` or `request.signal` to `exec()` directly, because either can fire after the process exits. Pass the signal of an `AbortController` instead. Once `exitCode` resolves, stop anything that calls `abort()`, for example by clearing the timer or removing the `abort` listener from `request.signal`.

#### Exceptions

- `exec()` throws when the container is not running.
- `exec()` throws a `TypeError` when `cmd` is empty, an option mode is invalid, `stderr: "combined"` is used with `stdout: "ignore"`, or `pty` is used with a `stderr` value other than `"combined"`.
- `exec()` throws an `AbortError` when `signal` is already aborted.
- `exec()` rejects if the runtime cannot create or start the process.
- Environment variable names cannot contain `=` or null characters. Environment values, `cwd`, and `user` cannot contain null characters.
- `kill()` throws a `RangeError` when the signal is outside the supported range.
- `resize()` throws a `TypeError` when the process does not have a pseudo-terminal, and a `RangeError` when a dimension is out of range.

For task-oriented examples, refer to [Execute commands](https://developers.cloudflare.com/containers/guides/execute-commands/).

### `snapshotContainer` durable\_object policy only

`snapshotContainer()` creates a point-in-time snapshot of the writable root filesystem of a running container. It does not capture memory, running processes, or separately mounted filesystems. A container started from the snapshot runs its entrypoint again. This method is only supported for applications that use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy).

```js
const snapshot = await this.ctx.container.snapshotContainer({
	name: "before-upgrade",
});
```

Note

The Worker API has no method to list snapshots. Store the returned `id`, for example in Durable Object storage.

#### Parameters

- `options` ( `ContainerSnapshotOptions`): Snapshot configuration. Pass `{}` when you do not set a name:
  - `name` ( `string`, optional): Human-readable name for the snapshot.

#### Return values

- `Promise<ContainerSnapshot>`: Resolves with an opaque handle for the stored filesystem snapshot. The snapshot data is not returned to the Worker. The handle contains:
  - `id` ( `string`): Unique snapshot identifier. Pass the returned `ContainerSnapshot` to [`start()`](#start) as `containerSnapshot`, or store it for a later restore.
  - `size` ( `number`): Snapshot size in bytes.
  - `name` ( `string`, optional): Human-readable name supplied in `options`.

Container snapshots are immutable. Snapshot handles have an implicit 30-day time-to-live that refreshes when you restore them. For the complete save and restore flow, refer to [Use snapshots](https://developers.cloudflare.com/containers/guides/snapshots/).

#### Exceptions

- `snapshotContainer()` throws an `Error` when the container is not running.
- `snapshotContainer()` throws a `TypeError` when `options` is omitted.

### `destroy`

`destroy()` stops the container and can include an optional reason for the operation.

```js
await this.ctx.container.destroy("Manually Destroyed");
```

#### Parameters

- `error` ( `any`, optional): Optional reason associated with the destroy operation. A string is commonly used for logging or debugging.

#### Return values

- `Promise<void>`: Resolves when the container is destroyed.

### `signal`

`signal()` sends an inter-process communication (IPC) signal to the container, such as `SIGKILL` or `SIGTERM`. Use it to stop the container gracefully or forcefully.

```js
const SIGTERM = 15;
this.ctx.container.signal(SIGTERM);
```

#### Parameters

- `signal` ( `number`): POSIX signal number to send to the container, such as `SIGTERM` ( `15`) or `SIGKILL` ( `9`).

#### Return values

- `void`: No return value.

### `setInactivityTimeout`

`setInactivityTimeout()` sets how long a container keeps running after its Durable Object becomes inactive. If the Durable Object handles another request before the timeout ends, the same container is still running. Otherwise, Cloudflare stops the container. Without a timeout, Cloudflare stops the container shortly after the Durable Object becomes inactive.

Each Durable Object instance sets its own timeout. A Durable Object that restarts, for example after a deploy, starts without one. Set the timeout after `start()`, and set it again in the constructor when the container is already running.

```txt
setInactivityTimeout(durationMs: number | bigint): Promise<void>
```

*index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

const INACTIVITY_TIMEOUT_MS = 10 * 60 * 1000;

export class MyDurableObject extends DurableObject {
	constructor(ctx, env) {
		super(ctx, env);
		const container = ctx.container;
		if (container?.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async ensureRunning() {
		const container = this.ctx.container;
		if (!container) {
			throw new Error("No container is configured for this Durable Object");
		}
		if (!container.running) {
			container.start({
				// `cloudflare/debian-trixie` requires the `durable_object` scheduling policy.
				image: "cloudflare/debian-trixie",
				entrypoint: ["sleep", "infinity"],
				enableInternet: false,
			});
			await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);
		}
	}
}
```

*index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {}

const INACTIVITY_TIMEOUT_MS = 10 * 60 * 1000;

export class MyDurableObject extends DurableObject<Env> {
	constructor(ctx: DurableObjectState, env: Env) {
		super(ctx, env);
		const container = ctx.container;
		if (container?.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async ensureRunning() {
		const container = this.ctx.container;
		if (!container) {
			throw new Error("No container is configured for this Durable Object");
		}
		if (!container.running) {
			container.start({
				// `cloudflare/debian-trixie` requires the `durable_object` scheduling policy.
				image: "cloudflare/debian-trixie",
				entrypoint: ["sleep", "infinity"],
				enableInternet: false,
			});
			await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);
		}
	}
}
```

#### Parameters

- `durationMs` ( `number | bigint`): Inactivity timeout in milliseconds. The value must be greater than `0` and at most 6 hours ( `21600000`).

#### Return values

- `Promise<void>`: Resolves after the timeout is set.

#### Exceptions

- `setInactivityTimeout()` throws a `TypeError` when `durationMs` is `0` or less.
- The returned promise rejects when `durationMs` is greater than 6 hours.

Await the call inside a `try` block to handle both.

### `getTcpPort`

`getTcpPort()` returns a TCP port from the container. Use it to communicate with the container over TCP or HTTP.

```js
const port = this.ctx.container.getTcpPort(8080);
const res = await port.fetch("http://container/set-state", {
	body: initialState,
	method: "POST",
});
```

```js
const conn = this.ctx.container.getTcpPort(8080).connect("10.0.0.1:8080");
await conn.opened;

try {
	if (request.body) {
		await request.body.pipeTo(conn.writable);
	}
	return new Response(conn.readable);
} catch (error) {
	console.error("Request body piping failed:", error);
	return new Response("Failed to proxy request body", { status: 502 });
}
```

#### Parameters

- `port` ( `number`): TCP port number to use for communication with the container.

#### Return values

- `Fetcher`: Object used to send HTTP requests or TCP connections to the container port.

### `monitor`

`monitor()` returns a promise that resolves when a container exits and rejects if the container errors. Use it to handle container status changes in your Workers code.

A pending `monitor()` call prevents eviction until the container exits or for up to 15 minutes, whichever comes first. Starting another operation later can extend the Durable Object's [time in memory](https://developers.cloudflare.com/durable-objects/concepts/durable-object-lifecycle/).

```js
import { DurableObject } from "cloudflare:workers";

class MyDurableObject extends DurableObject {
	startAndMonitor() {
		const container = this.ctx.container;
		// With the `default` scheduling policy, call `container.start()` without options.
		container.start({
			image: container.images.base,
			enableInternet: false,
		});
		this.ctx.waitUntil(
			container
				.monitor()
				.then(() => console.log("Container exited"))
				.catch((error) => console.error("Container errored", error)),
		);
	}
}
```

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {}

class MyDurableObject extends DurableObject<Env> {
	startAndMonitor() {
		const container = this.ctx.container;
		// With the `default` scheduling policy, call `container.start()` without options.
		container.start({
			image: container.images.base,
			enableInternet: false,
		});
		this.ctx.waitUntil(
			container
				.monitor()
				.then(() => console.log("Container exited"))
				.catch((error) => console.error("Container errored", error)),
		);
	}
}
```

A `monitor()` promise does not carry over when the Durable Object restarts, for example after a deploy. If `running` is `true` when the Durable Object starts, call `monitor()` again to observe the container.

#### Parameters

- None.

#### Return values

- `Promise<void>`: Settles when the container stops, including a container that stopped before the call. The outcome depends on how the container stopped:
  - It resolves when the main process exits with code `0`, or when `destroy()` stops the container without an error value.
  - It rejects with an `Error` that has an `exitCode` property when the main process exits with another code.
  - It rejects with the value passed to `destroy()`, when there is one.
  - It rejects with an error that describes the failure when the container stops for another reason.

#### Exceptions

- `monitor()` throws an `Error` when this Durable Object instance has no container. An instance has a container after it calls `start()`, or when a container was already running as the instance started.

### `interceptOutboundHttp`

`interceptOutboundHttp()` routes outbound HTTP requests matching a hostname, hostname glob, IP address, IP:port, or CIDR range through a `Fetcher`. Call it before or after starting the container. An intercept lasts until the container stops, so register intercepts again for each new container. Registering a target again replaces its handler, and open connections use the new handler without being dropped.

```js
const worker = this.ctx.exports.MyWorker({ props: { message: "hello" } });

// Match a specific hostname
await this.ctx.container.interceptOutboundHttp("api.example.com", worker);

// Match a hostname glob pattern
await this.ctx.container.interceptOutboundHttp("*.example.com", worker);

// Match an IP:port
await this.ctx.container.interceptOutboundHttp("15.0.0.1:80", worker);

// Match an IPv4 CIDR range. Register an IPv6 range separately.
await this.ctx.container.interceptOutboundHttp("203.0.113.0/24", worker);
```

In a container started with `enableInternet: false`, no public resolver answers DNS lookups. Lookups resolve as follows:

- `A` and `AAAA` lookups for an intercepted hostname return a placeholder address that routes the request to the intercept.
- `interceptAllOutboundHttp()` and a `*` hostname target make every hostname resolve, including hostnames that do not exist.
- IP address and CIDR targets add no DNS answers, except for `0.0.0.0/0` and `::/0`, which make every hostname resolve.
- Every other lookup times out, including a lookup made before you register an intercept.

#### Intercept limit

A container has 128 intercept entries, which `interceptOutboundHttp()`, `interceptOutboundHttps()`, and `interceptAllOutboundHttp()` share. Each new target uses entries as follows:

- A hostname or hostname glob, including `*`, uses two entries, one for IPv4 and one for IPv6.
- An IP address, IP:port, or CIDR range uses one entry.
- The `interceptAllOutboundHttp()` rule uses two entries.

A hostname intercepted for both HTTP and HTTPS is two targets and uses four entries. A container can hold up to 64 hostname targets, or up to 128 IP address and CIDR targets. Registering an existing target again uses no more entries.

You cannot remove an intercept while the container runs. To route a changing set of hostnames, register `interceptAllOutboundHttp()` or the `*` target once. Then choose the behavior for each hostname inside the `WorkerEntrypoint`, for example from its props. To change the props, register the rule again with new props.

When a new target does not fit, awaiting the call throws an `Error` with the message `You can't configure more than 128 egress interceptors`. Intercepts registered earlier keep working.

#### Parameters

- `addr` ( `string`): Hostname, hostname glob (for example, `*.example.com`), IP address, IP:port, or CIDR range to match.
- `binding` ( `Fetcher`): Worker entrypoint or service binding that handles matching requests.

#### Return values

- `Promise<void>`: Resolves when the intercept rule is installed.

#### Exceptions

- Awaiting `interceptOutboundHttp()` throws an `Error` when `addr` is new and the container does not have enough free entries for it. For more information, refer to [Intercept limit](#intercept-limit).

### `interceptAllOutboundHttp`

`interceptAllOutboundHttp()` routes outbound HTTP requests on port `80` from the container through a `Fetcher`, regardless of destination.

This rule does not intercept other ports, including HTTPS on port `443`. To intercept HTTPS as well, also register [`interceptOutboundHttps("*")`](#interceptoutboundhttps). In a container started with `enableInternet: true`, connections that no intercept covers reach the Internet directly. To block them, start the container with `enableInternet: false`.

Register hostname intercepts before `interceptAllOutboundHttp()`. A hostname intercept that [`interceptOutboundHttp()`](#interceptoutboundhttp) registers first keeps receiving its requests, even after you register this rule again. A hostname intercept registered after this rule receives no requests.

```js
await this.ctx.container.interceptAllOutboundHttp(worker);
```

#### Parameters

- `binding` ( `Fetcher`): Worker entrypoint or service binding that handles all outbound HTTP requests.

#### Return values

- `Promise<void>`: Resolves when the intercept rule is installed.

#### Exceptions

- Awaiting `interceptAllOutboundHttp()` throws an `Error` when the rule is not registered yet and the container does not have two free entries. For more information, refer to [Intercept limit](#intercept-limit).

### `interceptOutboundHttps`

`interceptOutboundHttps()` routes outbound HTTPS requests matching a hostname or hostname glob, with an optional port, through a `Fetcher`. It works like `interceptOutboundHttp()` but handles HTTPS traffic. The container must trust the CA certificate at `/etc/cloudflare/certs/cloudflare-containers-ca.crt` for HTTPS interception.

Hostname globs support `*` to match any sequence of characters.

```js
const worker = this.ctx.exports.MyWorker({ props: {} });

// Match a specific hostname
await this.ctx.container.interceptOutboundHttps("api.example.com", worker);

// Match a hostname glob pattern
await this.ctx.container.interceptOutboundHttps("*.example.com", worker);

// Intercept all HTTPS traffic
await this.ctx.container.interceptOutboundHttps("*", worker);
```

`interceptOutboundHttps()` intercepts HTTPS on port `443` by default. To use another port, include it in the target, for example `"api.example.com:8443"`. It matches the hostname that the client sends. In a container started with `enableInternet: false`, HTTPS requests to a bare IP address fail while a `*` intercept is registered.

#### Parameters

- `addr` ( `string`): Hostname or hostname glob pattern to match, with an optional port such as `api.example.com:8443`. Use `*` to intercept all HTTPS traffic on port `443`.
- `binding` ( `Fetcher`): Worker entrypoint or service binding that handles matching requests.

#### Return values

- `Promise<void>`: Resolves when the intercept rule is installed.

#### Exceptions

- Awaiting `interceptOutboundHttps()` throws an `Error` when `addr` is new and the container does not have enough free entries for it. For more information, refer to [Intercept limit](#intercept-limit).

## Related resources

- [Containers APIs](https://developers.cloudflare.com/containers/api/): Compare direct runtime control with the `Container` class.
- [Container class reference](https://developers.cloudflare.com/containers/api/container-class/): Reference for existing `Container` class applications.
- [Containers overview](https://developers.cloudflare.com/containers/): Understand how Cloudflare Containers work.
- [Get started with Containers](https://developers.cloudflare.com/containers/get-started/): Deploy your first container.
- [SQLite storage API](https://developers.cloudflare.com/durable-objects/api/sqlite-storage-api/): Persist state across container restarts.
- [Durable Objects](https://developers.cloudflare.com/durable-objects/): The underlying platform that powers Containers.
- [Snapshots](https://developers.cloudflare.com/containers/guides/snapshots/): Save and restore container filesystems.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/api/durable-object-container/#page","headline":"Durable Object Container API","description":"Access and manage containers associated with a Durable Object, including start, stop, and interaction methods.","url":"https://developers.cloudflare.com/containers/api/durable-object-container/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/api/durable-object-container/og.png?v=f13dbe1f95442c69","dateModified":"2026-09-30","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Code examples showing how to use Containers with Workers for stateless routing, cron jobs, WebSockets, and more.
title: Examples
image: https://developers.cloudflare.com/containers/examples/og.png?v=e9cf0d7d69da226c
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Examples

Last updated Oct 2, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Build with Containers using the `durable_object` scheduling policy. Each example configures a named image and starts it from a Durable Object with the [Container API](https://developers.cloudflare.com/containers/api/durable-object-container/).

Start with [Stateless instances](https://developers.cloudflare.com/containers/examples/stateless/) for HTTP routing, or [Cron container](https://developers.cloudflare.com/containers/examples/cron/) for a scheduled task.

[**Static frontend, container backend** A simple frontend app with a containerized backend](https://developers.cloudflare.com/containers/examples/container-backend/)

[**Cron container** Running a container on a schedule using Cron Triggers](https://developers.cloudflare.com/containers/examples/cron/)

[**More examples** Various examples calling Containers directly from Durable Objects](https://developers.cloudflare.com/containers/examples/durable-object-interface/)

[**Environment variables and secrets** Pass in environment variables and secrets to your container](https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/)

[**Mount R2 buckets with FUSE** Mount R2 buckets as filesystems using FUSE in Containers](https://developers.cloudflare.com/containers/examples/r2-fuse-mount/)

[**Stateless instances** Run multiple instances across Cloudflare's network](https://developers.cloudflare.com/containers/examples/stateless/)

[**Monitor container lifecycle** Execute Workers code in reaction to Container status changes](https://developers.cloudflare.com/containers/examples/status-hooks/)

[**WebSocket to container** Forwarding a Websocket request to a Container](https://developers.cloudflare.com/containers/examples/websocket/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/examples/#page","headline":"Examples","description":"Code examples showing how to use Containers with Workers for stateless routing, cron jobs, WebSockets, and more.","url":"https://developers.cloudflare.com/containers/examples/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/og.png?v=e9cf0d7d69da226c","dateModified":"2026-10-02","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: A simple frontend app with a containerized backend
title: Static frontend, container backend
image: https://developers.cloudflare.com/containers/examples/container-backend/og.png?v=bad3f19d23bc5627
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Static frontend, container backend

A simple frontend app with a containerized backend

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/container-backend/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Serve a static frontend with [Workers Static Assets](https://developers.cloudflare.com/workers/static-assets/). Route API requests to a pool of three Containers using the `durable_object` scheduling policy.

## Configure the Worker

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "static-frontend-container-backend",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "assets": {
    "directory": "./dist",
    "binding": "ASSETS",
    "run_worker_first": [
      "/api",
      "/api/*"
    ]
  },
  "containers": [
    {
      "class_name": "Backend",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "BACKEND",
        "class_name": "Backend"
      }
    ]
  },
  "exports": {
    "Backend": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  }
}
```

```toml
name = "static-frontend-container-backend"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[assets]
directory = "./dist"
binding = "ASSETS"
run_worker_first = ["/api", "/api/*"]

[[containers]]
class_name = "Backend"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "BACKEND"
class_name = "Backend"

[exports.Backend]
type = "durable-object"
storage = "sqlite"
```

## Add the frontend

Create a simple `index.html` file in the `./dist` directory.

<details>

<summary>

index.html

</summary>

```html
<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="UTF-8" />
		<meta name="viewport" content="width=device-width, initial-scale=1.0" />
		<title>Widgets</title>
		<script
			defer
			src="https://cdnjs.cloudflare.com/ajax/libs/alpinejs/3.13.3/cdn.min.js"
		></script>
	</head>

	<body>
		<div x-data="widgets()" x-init="fetchWidgets()">
			<h1>Widgets</h1>
			<div x-show="loading">Loading...</div>
			<div x-show="error" x-text="error" style="color: red;"></div>
			<ul x-show="!loading && !error">
				<template x-for="widget in widgets" :key="widget.id">
					<li>
						<span x-text="widget.name"></span> - (ID:
						<span x-text="widget.id"></span>)
					</li>
				</template>
			</ul>

			<div x-show="!loading && !error && widgets.length === 0">
				No widgets found.
			</div>
		</div>

		<script>
			function widgets() {
				return {
					widgets: [],
					loading: false,
					error: null,

					async fetchWidgets() {
						this.loading = true;
						this.error = null;

						try {
							const response = await fetch("/api/widgets");
							if (!response.ok) {
								throw new Error(
									`HTTP ${response.status}: ${response.statusText}`,
								);
							}
							this.widgets = await response.json();
						} catch (err) {
							this.error = err.message;
						} finally {
							this.loading = false;
						}
					},
				};
			}
		</script>
	</body>
</html>
```

</details>

The frontend uses [Alpine.js ↗︎](https://alpinejs.dev/) to fetch a list of widgets from `/api/widgets`.

This is meant to be a very simple example, but you can get significantly more complex. See [examples of Workers integrating with frontend frameworks](https://developers.cloudflare.com/workers/framework-guides/web-apps/) for more information.

## Route API requests

`start()` initiates startup. The Worker waits for a successful `/health` response before forwarding traffic. Concurrent requests share that readiness check, and a failed check can be retried by the next request.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

const INSTANCE_COUNT = 3;

const INACTIVITY_TIMEOUT_MS = 2 * 60 * 60 * 1000;

export class Backend extends DurableObject {
	starting;

	constructor(ctx, env) {
		super(ctx, env);
		const container = ctx.container;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request) {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container.getTcpPort(8080).fetch(forwarded);
	}

	async startAndWaitForPort() {
		const container = this.ctx.container;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request, env) {
		const pathname = new URL(request.url).pathname;
		if (pathname === "/api" || pathname.startsWith("/api/")) {
			const index = Math.floor(Math.random() * INSTANCE_COUNT);
			return env.BACKEND.getByName(`instance-${index}`).fetch(request);
		}
		return env.ASSETS.fetch(request);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	BACKEND: DurableObjectNamespace<Backend>;
	ASSETS: Fetcher;
}

const INSTANCE_COUNT = 3;

const INACTIVITY_TIMEOUT_MS = 2 * 60 * 60 * 1000;

export class Backend extends DurableObject<Env> {
	private starting: Promise<void> | undefined;

	constructor(ctx: DurableObjectState, env: Env) {
		super(ctx, env);
		const container = ctx.container!;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request: Request): Promise<Response> {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container!.getTcpPort(8080).fetch(forwarded);
	}

	private async startAndWaitForPort(): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError: unknown;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request: Request, env: Env): Promise<Response> {
		const pathname = new URL(request.url).pathname;
		if (pathname === "/api" || pathname.startsWith("/api/")) {
			const index = Math.floor(Math.random() * INSTANCE_COUNT);
			return env.BACKEND.getByName(`instance-${index}`).fetch(request);
		}
		return env.ASSETS.fetch(request);
	},
} satisfies ExportedHandler<Env>;
```

The Worker selects one of three named instances for each API request. Each instance uses the configured `base` image and the `lite` instance size.

## Define a backend container

Your container should be able to handle requests to `/api/widgets`.

In this case, we'll use a simple Golang backend that returns a hard-coded list of widgets.

<details>

<summary>

server.go

</summary>

```go
package main

import (
	"encoding/json"
	"log"
	"net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
	widgets := []map[string]interface{}{
		{"id": 1, "name": "Widget A"},
		{"id": 2, "name": "Sprocket B"},
		{"id": 3, "name": "Gear C"},
	}

	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	json.NewEncoder(w).Encode(widgets)

}

func main() {
	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	})
	http.HandleFunc("/api/widgets", handler)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

</details>

The health endpoint lets the Worker wait for the backend before forwarding requests. Build the backend image with this Dockerfile in the project root:

<details>

<summary>

Dockerfile

</summary>

```dockerfile
FROM golang:1.25-alpine AS build
WORKDIR /app
COPY server.go .
RUN CGO_ENABLED=0 go build -o /server server.go

FROM alpine:3.20
COPY --from=build /server /server
EXPOSE 8080
CMD ["/server"]
```

</details>

## Run the example

Start a Docker-compatible engine. Install Wrangler in your project, using version 4.136.0 or later for [local development](https://developers.cloudflare.com/containers/guides/local-dev/).

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev
```

```
yarn wrangler dev
```

```
pnpm wrangler dev
```

Visit `http://localhost:8787/` to load the frontend and its list of widgets.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/container-backend/#page","headline":"Static frontend, container backend","description":"A simple frontend app with a containerized backend","url":"https://developers.cloudflare.com/containers/examples/container-backend/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/container-backend/og.png?v=bad3f19d23bc5627","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Running a container on a schedule using Cron Triggers
title: Cron container
image: https://developers.cloudflare.com/containers/examples/cron/og.png?v=8aa941b4f4fd1c08
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Cron container

Running a container on a schedule using Cron Triggers

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/cron/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Run a short container task every two minutes with a Workers [Cron Trigger](https://developers.cloudflare.com/workers/configuration/cron-triggers/). The task prints its scheduled time, then exits.

## Configure the schedule

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "cron-container",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "triggers": {
    "crons": [
      "*/2 * * * *"
    ]
  },
  "containers": [
    {
      "class_name": "CronContainer",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "CRON_CONTAINER",
        "class_name": "CronContainer"
      }
    ]
  },
  "exports": {
    "CronContainer": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  }
}
```

```toml
name = "cron-container"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[triggers]
crons = ["*/2 * * * *"]

[[containers]]
class_name = "CronContainer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "CRON_CONTAINER"
class_name = "CronContainer"

[exports.CronContainer]
type = "durable-object"
storage = "sqlite"
```

## Start and await the task

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class CronContainer extends DurableObject {
	currentRun;

	run(startTime) {
		this.currentRun ??= this.runOnce(startTime).finally(() => {
			this.currentRun = undefined;
		});
		return this.currentRun;
	}

	async runOnce(startTime) {
		const container = this.ctx.container;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
				env: { MESSAGE: `Scheduled time: ${startTime}` },
			});
		}
		await container.monitor();
	}
}

export default {
	fetch() {
		return new Response("This Worker runs a scheduled Container task.");
	},
	async scheduled(controller, env) {
		await env.CRON_CONTAINER.getByName("cron").run(
			new Date(controller.scheduledTime).toISOString(),
		);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	CRON_CONTAINER: DurableObjectNamespace<CronContainer>;
}

export class CronContainer extends DurableObject<Env> {
	private currentRun: Promise<void> | undefined;

	run(startTime: string): Promise<void> {
		this.currentRun ??= this.runOnce(startTime).finally(() => {
			this.currentRun = undefined;
		});
		return this.currentRun;
	}

	private async runOnce(startTime: string): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
				env: { MESSAGE: `Scheduled time: ${startTime}` },
			});
		}
		await container.monitor();
	}
}

export default {
	fetch(): Response {
		return new Response("This Worker runs a scheduled Container task.");
	},
	async scheduled(controller: ScheduledController, env: Env): Promise<void> {
		await env.CRON_CONTAINER.getByName("cron").run(
			new Date(controller.scheduledTime).toISOString(),
		);
	},
} satisfies ExportedHandler<Env>;
```

All triggers use the same Durable Object. If a run is still active, another trigger waits for that run instead of starting a second task. This example does not queue overlapping runs or guarantee exactly-once execution. Make tasks idempotent when they modify external data.

`monitor()` reports completion or failure, not HTTP readiness. Keep this task short enough to finish within the [Cron Trigger execution limits](https://developers.cloudflare.com/workers/platform/limits/#duration).

## Define the task

*Dockerfiledockerfile*

```dockerfile
FROM alpine:3.20
CMD ["sh", "-c", "printf '%s\\n' \"$MESSAGE\""]
```

## Test the schedule locally

Start a Docker-compatible engine and use Wrangler 4.136.0 or later.

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev --test-scheduled
```

```
yarn wrangler dev --test-scheduled
```

```
pnpm wrangler dev --test-scheduled
```

In another terminal, trigger the scheduled handler:

```sh
curl 'http://localhost:8787/__scheduled?cron=*/2+*+*+*+*'
```

Check the container logs for the scheduled time. After deployment, Cron Triggers invoke the handler automatically.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/cron/#page","headline":"Cron container","description":"Running a container on a schedule using Cron Triggers","url":"https://developers.cloudflare.com/containers/examples/cron/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/cron/og.png?v=8aa941b4f4fd1c08","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Various examples calling Containers directly from Durable Objects
title: More examples
image: https://developers.cloudflare.com/containers/examples/durable-object-interface/og.png?v=2fb49aba202d1f04
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# More examples

Various examples calling Containers directly from Durable Objects

Last updated Oct 2, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/durable-object-interface/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Explore more ways to control a Container from its Durable Object:

- [Execute commands](https://developers.cloudflare.com/containers/guides/execute-commands/): Run a process and read its output.
- [Use snapshots](https://developers.cloudflare.com/containers/guides/snapshots/): Save a filesystem and restore it when starting a Container.
- [Roll out a named image update](https://developers.cloudflare.com/containers/guides/image-management/#roll-out-a-named-image-update): Compare the running image with the configured image, then restart when needed.

Use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy) and the [Durable Object Container API](https://developers.cloudflare.com/containers/api/durable-object-container/) for these workflows.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/durable-object-interface/#page","headline":"More examples","description":"Various examples calling Containers directly from Durable Objects","url":"https://developers.cloudflare.com/containers/examples/durable-object-interface/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/durable-object-interface/og.png?v=2fb49aba202d1f04","dateModified":"2026-10-02","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Pass in environment variables and secrets to your container
title: Environment variables and secrets
image: https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/og.png?v=d15710cc7365b856
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Environment variables and secrets

Pass in environment variables and secrets to your container

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Pass runtime environment variables through `env` in `ctx.container.start()`. Read Worker secrets, Secrets Store values, and KV data inside the Durable Object before starting its Container.

This example passes four values to the container process: `ENV_VAR`, `WORKER_SECRET`, `SECRET_STORE_SECRET`, and `KV_VALUE`. The HTTP response includes only the two non-secret values.

## Configure bindings

Create a [KV namespace](https://developers.cloudflare.com/kv/get-started/) and a [Secrets Store secret](https://developers.cloudflare.com/secrets-store/integrations/workers/). Use a secret named `SECRET_STORE_SECRET` with the `workers` scope. Replace the namespace and store IDs in this configuration with the IDs returned during setup.

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "container-environment",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "containers": [
    {
      "class_name": "MyContainer",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "MY_CONTAINER",
        "class_name": "MyContainer"
      }
    ]
  },
  "exports": {
    "MyContainer": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  },
  "vars": {
    "ENV_VAR": "demo"
  },
  "kv_namespaces": [
    {
      "binding": "DEMO_KV",
      "id": "<KV_NAMESPACE_ID>"
    }
  ],
  "secrets_store_secrets": [
    {
      "binding": "SECRET_STORE",
      "store_id": "<STORE_ID>",
      "secret_name": "SECRET_STORE_SECRET"
    }
  ]
}
```

```toml
name = "container-environment"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[[containers]]
class_name = "MyContainer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"

[vars]
ENV_VAR = "demo"

[[kv_namespaces]]
binding = "DEMO_KV"
id = "<KV_NAMESPACE_ID>"

[[secrets_store_secrets]]
binding = "SECRET_STORE"
store_id = "<STORE_ID>"
secret_name = "SECRET_STORE_SECRET"
```

Create the Worker secret, then populate the KV key for deployment:

npmyarnpnpm

```
npx wrangler secret put WORKER_SECRET
```

```
yarn wrangler secret put WORKER_SECRET
```

```
pnpm wrangler secret put WORKER_SECRET
```

npmyarnpnpm

```
npx wrangler kv key put --binding DEMO_KV KV_VALUE 'Hello from KV!' --remote
```

```
yarn wrangler kv key put --binding DEMO_KV KV_VALUE 'Hello from KV!' --remote
```

```
pnpm wrangler kv key put --binding DEMO_KV KV_VALUE 'Hello from KV!' --remote
```

`WORKER_SECRET` is available through the Worker environment without a `vars` entry. Store IDs identify stores; they are not store names.

## Pass values at startup

`start()` initiates startup. The Worker waits for a successful `/health` response before forwarding traffic. Concurrent requests share that readiness check, and a failed check can be retried by the next request.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

const INACTIVITY_TIMEOUT_MS = 60 * 1000;

export class MyContainer extends DurableObject {
	starting;

	constructor(ctx, env) {
		super(ctx, env);
		const container = ctx.container;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request) {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container.getTcpPort(8080).fetch(forwarded);
	}

	async startAndWaitForPort() {
		const container = this.ctx.container;
		if (!container.running) {
			const [storedSecret, kvValue] = await Promise.all([
				this.env.SECRET_STORE.get(),
				this.env.DEMO_KV.get("KV_VALUE"),
			]);
			if (storedSecret === null || kvValue === null) {
				throw new Error(
					"Create the secret and KV value before starting the container",
				);
			}
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
				env: {
					ENV_VAR: this.env.ENV_VAR,
					WORKER_SECRET: this.env.WORKER_SECRET,
					SECRET_STORE_SECRET: storedSecret,
					KV_VALUE: kvValue,
				},
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request, env) {
		return env.MY_CONTAINER.getByName("demo").fetch(request);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	MY_CONTAINER: DurableObjectNamespace<MyContainer>;
	ENV_VAR: string;
	WORKER_SECRET: string;
	SECRET_STORE: SecretsStoreSecret;
	DEMO_KV: KVNamespace;
}

const INACTIVITY_TIMEOUT_MS = 60 * 1000;

export class MyContainer extends DurableObject<Env> {
	private starting: Promise<void> | undefined;

	constructor(ctx: DurableObjectState, env: Env) {
		super(ctx, env);
		const container = ctx.container!;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request: Request): Promise<Response> {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container!.getTcpPort(8080).fetch(forwarded);
	}

	private async startAndWaitForPort(): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			const [storedSecret, kvValue] = await Promise.all([
				this.env.SECRET_STORE.get(),
				this.env.DEMO_KV.get("KV_VALUE"),
			]);
			if (storedSecret === null || kvValue === null) {
				throw new Error(
					"Create the secret and KV value before starting the container",
				);
			}
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
				env: {
					ENV_VAR: this.env.ENV_VAR,
					WORKER_SECRET: this.env.WORKER_SECRET,
					SECRET_STORE_SECRET: storedSecret,
					KV_VALUE: kvValue,
				},
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError: unknown;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request: Request, env: Env): Promise<Response> {
		return env.MY_CONTAINER.getByName("demo").fetch(request);
	},
} satisfies ExportedHandler<Env>;
```

Each named Durable Object reads the values when starting its Container. A running process keeps its startup environment. Updating a secret or KV value does not change that process's environment; the updated value is read on its next start.

## Define the container

Save these files in the project root. The server listens on `0.0.0.0:8080` and exposes a readiness endpoint at `/health`.

*server.mjsjs*

```js
import { createServer } from "node:http";

createServer((request, response) => {
	if (request.url === "/health") {
		response.writeHead(200).end();
		return;
	}
	response.setHeader("Content-Type", "application/json");
	response.end(
		JSON.stringify({
			message: "Hello from a Container",
			environment: process.env.ENV_VAR,
			kvValue: process.env.KV_VALUE,
		}),
	);
}).listen(8080, "0.0.0.0");
```

*Dockerfiledockerfile*

```dockerfile
FROM node:24-bookworm-slim
WORKDIR /app
COPY server.mjs .
EXPOSE 8080
CMD ["node", "server.mjs"]
```

## Test locally

For local development, create a `.dev.vars` file with test values. Do not commit this file.

*.dev.varstxt*

```txt
WORKER_SECRET="local-worker-secret"
```

Create a local Secrets Store value using the same store ID and secret name as the configuration. Enter a test value at the prompt:

npmyarnpnpm

```
npx wrangler secrets-store secret create <STORE_ID> --name SECRET_STORE_SECRET --scopes workers
```

```
yarn wrangler secrets-store secret create <STORE_ID> --name SECRET_STORE_SECRET --scopes workers
```

```
pnpm wrangler secrets-store secret create <STORE_ID> --name SECRET_STORE_SECRET --scopes workers
```

This command creates local data. It does not read or update the deployed secret.

Populate the local KV namespace:

npmyarnpnpm

```
npx wrangler kv key put --binding DEMO_KV KV_VALUE 'Hello from local KV!' --local
```

```
yarn wrangler kv key put --binding DEMO_KV KV_VALUE 'Hello from local KV!' --local
```

```
pnpm wrangler kv key put --binding DEMO_KV KV_VALUE 'Hello from local KV!' --local
```

Start a Docker-compatible engine. Install Wrangler in your project, using version 4.136.0 or later for [local development](https://developers.cloudflare.com/containers/guides/local-dev/).

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev
```

```
yarn wrangler dev
```

```
pnpm wrangler dev
```

Visit `http://localhost:8787/`. The response shows `demo` and the local KV value, without exposing either secret.

## Set build variables

Use `build_vars` on a named image for non-secret Docker build arguments. For example, replace the `containers` entry in the configuration and declare `ARG APP_VERSION` in your Dockerfile:

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "containers": [
    {
      "class_name": "MyContainer",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile",
          "build_vars": {
            "APP_VERSION": "1.0.0"
          }
        }
      }
    }
  ]
}
```

```toml
[[containers]]
class_name = "MyContainer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"
build_vars = { APP_VERSION = "1.0.0" }
```

Build arguments do not automatically become runtime environment variables. Keep secrets out of build arguments and container images.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/#page","headline":"Environment variables and secrets","description":"Pass in environment variables and secrets to your container","url":"https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/og.png?v=d15710cc7365b856","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Mount R2 buckets as filesystems using FUSE in Containers
title: Mount R2 buckets with FUSE
image: https://developers.cloudflare.com/containers/examples/r2-fuse-mount/og.png?v=b64ab882fb3a9b6a
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Mount R2 buckets with FUSE

Mount R2 buckets as filesystems using FUSE in Containers

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/r2-fuse-mount/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Mount an [R2 bucket](https://developers.cloudflare.com/r2/) with Filesystem in Userspace (FUSE). This example starts a Container, mounts the bucket read-only, lists its contents, and exits. The Durable Object awaits completion with `monitor()`.

FUSE provides filesystem access to object storage. It does not provide local-disk performance or full POSIX filesystem semantics. It is useful for reading datasets, models, and shared assets.

## Configure the Worker

Create a bucket and [R2 API credentials](https://developers.cloudflare.com/r2/api/tokens/) with Object Read access scoped to that bucket. Replace the bucket name and account ID in this configuration:

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "r2-fuse-container",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "containers": [
    {
      "class_name": "FUSEDemo",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "FUSE_DEMO",
        "class_name": "FUSEDemo"
      }
    ]
  },
  "exports": {
    "FUSEDemo": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  },
  "vars": {
    "R2_BUCKET_NAME": "<BUCKET_NAME>",
    "R2_ACCOUNT_ID": "<ACCOUNT_ID>"
  }
}
```

```toml
name = "r2-fuse-container"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[[containers]]
class_name = "FUSEDemo"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "FUSE_DEMO"
class_name = "FUSEDemo"

[exports.FUSEDemo]
type = "durable-object"
storage = "sqlite"

[vars]
R2_BUCKET_NAME = "<BUCKET_NAME>"
R2_ACCOUNT_ID = "<ACCOUNT_ID>"
```

Store the credentials as Worker secrets:

npmyarnpnpm

```
npx wrangler secret put AWS_ACCESS_KEY_ID
```

```
yarn wrangler secret put AWS_ACCESS_KEY_ID
```

```
pnpm wrangler secret put AWS_ACCESS_KEY_ID
```

npmyarnpnpm

```
npx wrangler secret put AWS_SECRET_ACCESS_KEY
```

```
yarn wrangler secret put AWS_SECRET_ACCESS_KEY
```

```
pnpm wrangler secret put AWS_SECRET_ACCESS_KEY
```

## Define the container

Save the Dockerfile and startup script in the project root. This image installs [tigrisfs ↗︎](https://github.com/tigrisdata/tigrisfs), an adapter for S3-compatible storage.

*Dockerfiledockerfile*

```dockerfile
FROM alpine:3.20
RUN apk add --no-cache ca-certificates fuse curl

ARG TIGRISFS_VERSION=1.2.1
RUN ARCH=$(uname -m) && \
    case "$ARCH" in x86_64) ARCH=amd64 ;; aarch64) ARCH=arm64 ;; esac && \
    curl -fL "https://github.com/tigrisdata/tigrisfs/releases/download/v${TIGRISFS_VERSION}/tigrisfs_${TIGRISFS_VERSION}_linux_${ARCH}.tar.gz" -o /tmp/tigrisfs.tar.gz && \
    tar -xzf /tmp/tigrisfs.tar.gz -C /usr/local/bin/ && \
    rm /tmp/tigrisfs.tar.gz && \
    chmod +x /usr/local/bin/tigrisfs

COPY startup.sh /startup.sh
CMD ["sh", "/startup.sh"]
```

*startup.shsh*

```sh
set -eu
mkdir -p /mnt/r2

/usr/local/bin/tigrisfs \
  --endpoint "https://${R2_ACCOUNT_ID}.r2.cloudflarestorage.com" \
  --region auto -o ro -f "$R2_BUCKET_NAME" /mnt/r2 &
fuse_pid=$!
trap 'fusermount -u /mnt/r2 2>/dev/null || true; kill "$fuse_pid" 2>/dev/null || true' EXIT

# Wait for the mount instead of reading an empty local directory.
attempt=0
until mountpoint -q /mnt/r2; do
  if ! kill -0 "$fuse_pid" 2>/dev/null; then
    echo "FUSE process exited before mounting" >&2
    exit 1
  fi
  attempt=$((attempt + 1))
  if [ "$attempt" -ge 30 ]; then
    echo "Timed out mounting R2" >&2
    exit 1
  fi
  sleep 1
done

ls -lah /mnt/r2
```

The main process exits after listing the bucket. For a long-running application, run its process after the mount is ready and keep the mount alive for the application's lifetime.

## Start the task

Pass credentials through `env` when starting the Container. Set `enableInternet: true` so tigrisfs can reach the R2 endpoint.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

export class FUSEDemo extends DurableObject {
	currentRun;

	run() {
		this.currentRun ??= this.runOnce().finally(() => {
			this.currentRun = undefined;
		});
		return this.currentRun;
	}

	async runOnce() {
		const container = this.ctx.container;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: true,
				env: {
					AWS_ACCESS_KEY_ID: this.env.AWS_ACCESS_KEY_ID,
					AWS_SECRET_ACCESS_KEY: this.env.AWS_SECRET_ACCESS_KEY,
					R2_BUCKET_NAME: this.env.R2_BUCKET_NAME,
					R2_ACCOUNT_ID: this.env.R2_ACCOUNT_ID,
				},
			});
		}
		await container.monitor();
	}
}

export default {
	async fetch(request, env) {
		if (new URL(request.url).pathname !== "/run") {
			return new Response("Not found", { status: 404 });
		}
		if (request.method !== "POST") {
			return new Response("Use POST /run", {
				status: 405,
				headers: { Allow: "POST" },
			});
		}
		await env.FUSE_DEMO.getByName("demo").run();
		return new Response("Bucket listing complete. Check the container logs.");
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	FUSE_DEMO: DurableObjectNamespace<FUSEDemo>;
	AWS_ACCESS_KEY_ID: string;
	AWS_SECRET_ACCESS_KEY: string;
	R2_BUCKET_NAME: string;
	R2_ACCOUNT_ID: string;
}

export class FUSEDemo extends DurableObject<Env> {
	private currentRun: Promise<void> | undefined;

	run(): Promise<void> {
		this.currentRun ??= this.runOnce().finally(() => {
			this.currentRun = undefined;
		});
		return this.currentRun;
	}

	private async runOnce(): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: true,
				env: {
					AWS_ACCESS_KEY_ID: this.env.AWS_ACCESS_KEY_ID,
					AWS_SECRET_ACCESS_KEY: this.env.AWS_SECRET_ACCESS_KEY,
					R2_BUCKET_NAME: this.env.R2_BUCKET_NAME,
					R2_ACCOUNT_ID: this.env.R2_ACCOUNT_ID,
				},
			});
		}
		await container.monitor();
	}
}

export default {
	async fetch(request: Request, env: Env): Promise<Response> {
		if (new URL(request.url).pathname !== "/run") {
			return new Response("Not found", { status: 404 });
		}
		if (request.method !== "POST") {
			return new Response("Use POST /run", {
				status: 405,
				headers: { Allow: "POST" },
			});
		}
		await env.FUSE_DEMO.getByName("demo").run();
		return new Response("Bucket listing complete. Check the container logs.");
	},
} satisfies ExportedHandler<Env>;
```

Concurrent calls share the active task. A later call starts another run after completion. Protect the route with authentication before exposing it to users.

## Test the mount

For local development, put the R2 credentials in `.dev.vars` as `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`. Do not commit that file. The local container connects to the configured R2 bucket.

Start a Docker-compatible engine that supports [FUSE during local development](https://developers.cloudflare.com/containers/guides/local-dev/#fuse-support). Use Wrangler 4.136.0 or later.

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev
```

```
yarn wrangler dev
```

```
pnpm wrangler dev
```

In another terminal, start the task:

```sh
curl -X POST http://localhost:8787/run
```

Check the container logs for the bucket listing. A mount failure makes `monitor()` reject instead of returning a successful response.

## Access a bucket prefix

After mounting, access a prefix through its filesystem path, such as `/mnt/r2/datasets`. A path within a mount does not restrict credentials to that prefix.

## Allow writes

To write through the mount, remove `-o ro` and use R2 credentials with Object Read and Write access. Wait for your application's writes and unmount cleanly before the container exits.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/r2-fuse-mount/#page","headline":"Mount R2 buckets with FUSE","description":"Mount R2 buckets as filesystems using FUSE in Containers","url":"https://developers.cloudflare.com/containers/examples/r2-fuse-mount/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/r2-fuse-mount/og.png?v=b64ab882fb3a9b6a","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Run multiple instances across Cloudflare's network
title: Stateless instances
image: https://developers.cloudflare.com/containers/examples/stateless/og.png?v=f01076b5fe66dd98
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Stateless instances

Run multiple instances across Cloudflare's network

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/stateless/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Distribute HTTP requests across three named Container instances. Each Durable Object starts its Container with the `durable_object` scheduling policy.

The Worker chooses among `instance-0`, `instance-1`, and `instance-2`. This is a fixed pool, not automatic scaling. Requests can reach different instances, so keep shared application data outside the container filesystem.

## Configure the Worker

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "stateless-containers",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "containers": [
    {
      "class_name": "Backend",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "BACKEND",
        "class_name": "Backend"
      }
    ]
  },
  "exports": {
    "Backend": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  }
}
```

```toml
name = "stateless-containers"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[[containers]]
class_name = "Backend"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "BACKEND"
class_name = "Backend"

[exports.Backend]
type = "durable-object"
storage = "sqlite"
```

## Route requests

`start()` initiates startup. The Worker waits for a successful `/health` response before forwarding traffic. Concurrent requests share that readiness check, and a failed check can be retried by the next request.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

const INSTANCE_COUNT = 3;

const INACTIVITY_TIMEOUT_MS = 2 * 60 * 60 * 1000;

export class Backend extends DurableObject {
	starting;

	constructor(ctx, env) {
		super(ctx, env);
		const container = ctx.container;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request) {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container.getTcpPort(8080).fetch(forwarded);
	}

	async startAndWaitForPort() {
		const container = this.ctx.container;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request, env) {
		const index = Math.floor(Math.random() * INSTANCE_COUNT);
		return env.BACKEND.getByName(`instance-${index}`).fetch(request);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	BACKEND: DurableObjectNamespace<Backend>;
}

const INSTANCE_COUNT = 3;

const INACTIVITY_TIMEOUT_MS = 2 * 60 * 60 * 1000;

export class Backend extends DurableObject<Env> {
	private starting: Promise<void> | undefined;

	constructor(ctx: DurableObjectState, env: Env) {
		super(ctx, env);
		const container = ctx.container!;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request: Request): Promise<Response> {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container!.getTcpPort(8080).fetch(forwarded);
	}

	private async startAndWaitForPort(): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError: unknown;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request: Request, env: Env): Promise<Response> {
		const index = Math.floor(Math.random() * INSTANCE_COUNT);
		return env.BACKEND.getByName(`instance-${index}`).fetch(request);
	},
} satisfies ExportedHandler<Env>;
```

## Define the container

Save these files in the project root. The server listens on `0.0.0.0:8080` and exposes a readiness endpoint at `/health`.

*server.mjsjs*

```js
import { createServer } from "node:http";

createServer((request, response) => {
	if (request.url === "/health") {
		response.writeHead(200).end();
		return;
	}
	response.setHeader("Content-Type", "application/json");
	response.end(
		JSON.stringify({
			message: "Hello from a Container",
			environment: process.env.ENV_VAR,
			kvValue: process.env.KV_VALUE,
		}),
	);
}).listen(8080, "0.0.0.0");
```

*Dockerfiledockerfile*

```dockerfile
FROM node:24-bookworm-slim
WORKDIR /app
COPY server.mjs .
EXPOSE 8080
CMD ["node", "server.mjs"]
```

## Run the example

Start a Docker-compatible engine. Install Wrangler in your project, using version 4.136.0 or later for [local development](https://developers.cloudflare.com/containers/guides/local-dev/).

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev
```

```
yarn wrangler dev
```

```
pnpm wrangler dev
```

Visit `http://localhost:8787/` to receive the container response. Instances stop after their Durable Objects remain inactive for two hours.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/stateless/#page","headline":"Stateless instances","description":"Run multiple instances across Cloudflare's network","url":"https://developers.cloudflare.com/containers/examples/stateless/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/stateless/og.png?v=f01076b5fe66dd98","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Execute Workers code in reaction to Container status changes
title: Monitor container lifecycle
image: https://developers.cloudflare.com/containers/examples/status-hooks/og.png?v=26c87d17da56027f
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Monitor container lifecycle

Execute Workers code in reaction to Container status changes

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/status-hooks/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Log when a Container becomes ready, exits successfully, or fails. The Durable Object starts a named image and uses `monitor()` to observe its exit.

## Configure the Worker

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "container-lifecycle",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "containers": [
    {
      "class_name": "MyContainer",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "MY_CONTAINER",
        "class_name": "MyContainer"
      }
    ]
  },
  "exports": {
    "MyContainer": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  }
}
```

```toml
name = "container-lifecycle"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[[containers]]
class_name = "MyContainer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"
```

## Observe readiness and exit

`start()` initiates startup. The Worker waits for a successful `/health` response before forwarding traffic. Concurrent requests share that readiness check, and a failed check can be retried by the next request.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

const INACTIVITY_TIMEOUT_MS = 5 * 60 * 1000;

export class MyContainer extends DurableObject {
	starting;
	monitoring;

	constructor(ctx, env) {
		super(ctx, env);
		const container = ctx.container;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
			this.observeExit();
		}
	}

	async fetch(request) {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container.getTcpPort(8080).fetch(forwarded);
	}

	async startAndWaitForPort() {
		const container = this.ctx.container;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		this.observeExit();
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				console.log("Container is ready");
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}

	observeExit() {
		if (this.monitoring) return;
		this.monitoring = this.ctx.container
			.monitor()
			.then(() => console.log("Container exited successfully"))
			.catch((error) => console.error("Container failed:", error))
			.finally(() => {
				this.monitoring = undefined;
			});
		this.ctx.waitUntil(this.monitoring);
	}
}

export default {
	fetch(request, env) {
		return env.MY_CONTAINER.getByName("demo").fetch(request);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	MY_CONTAINER: DurableObjectNamespace<MyContainer>;
}

const INACTIVITY_TIMEOUT_MS = 5 * 60 * 1000;

export class MyContainer extends DurableObject<Env> {
	private starting: Promise<void> | undefined;
	private monitoring: Promise<void> | undefined;

	constructor(ctx: DurableObjectState, env: Env) {
		super(ctx, env);
		const container = ctx.container!;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
			this.observeExit();
		}
	}

	async fetch(request: Request): Promise<Response> {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container!.getTcpPort(8080).fetch(forwarded);
	}

	private async startAndWaitForPort(): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		this.observeExit();
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError: unknown;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				console.log("Container is ready");
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}

	private observeExit(): void {
		if (this.monitoring) return;
		this.monitoring = this.ctx
			.container!.monitor()
			.then(() => console.log("Container exited successfully"))
			.catch((error: unknown) => console.error("Container failed:", error))
			.finally(() => {
				this.monitoring = undefined;
			});
		this.ctx.waitUntil(this.monitoring);
	}
}

export default {
	fetch(request: Request, env: Env): Promise<Response> {
		return env.MY_CONTAINER.getByName("demo").fetch(request);
	},
} satisfies ExportedHandler<Env>;
```

`monitor()` resolves without a value after a successful exit. It rejects for failures, including nonzero process exits. The constructor restores monitoring and the inactivity timeout when the Durable Object restarts with a running Container.

A pending monitor can keep the Durable Object active for up to 15 minutes. The five-minute inactivity timeout starts after the Durable Object becomes inactive, not five minutes after the last HTTP request. Monitoring is not a durable event subscription across restarts. Refer to [`monitor()`](https://developers.cloudflare.com/containers/api/durable-object-container/#monitor).

## Define the container

Save these files in the project root. The server listens on `0.0.0.0:8080` and exposes a readiness endpoint at `/health`.

*server.mjsjs*

```js
import { createServer } from "node:http";

createServer((request, response) => {
	if (request.url === "/health") {
		response.writeHead(200).end();
		return;
	}
	response.setHeader("Content-Type", "application/json");
	response.end(
		JSON.stringify({
			message: "Hello from a Container",
			environment: process.env.ENV_VAR,
			kvValue: process.env.KV_VALUE,
		}),
	);
}).listen(8080, "0.0.0.0");
```

*Dockerfiledockerfile*

```dockerfile
FROM node:24-bookworm-slim
WORKDIR /app
COPY server.mjs .
EXPOSE 8080
CMD ["node", "server.mjs"]
```

## Run the example

Start a Docker-compatible engine. Install Wrangler in your project, using version 4.136.0 or later for [local development](https://developers.cloudflare.com/containers/guides/local-dev/).

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev
```

```
yarn wrangler dev
```

```
pnpm wrangler dev
```

Visit `http://localhost:8787/` and inspect the Worker logs for readiness. Container completion or failure is logged when observed by `monitor()`.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/status-hooks/#page","headline":"Monitor container lifecycle","description":"Execute Workers code in reaction to Container status changes","url":"https://developers.cloudflare.com/containers/examples/status-hooks/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/status-hooks/og.png?v=26c87d17da56027f","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Forwarding a Websocket request to a Container
title: WebSocket to container
image: https://developers.cloudflare.com/containers/examples/websocket/og.png?v=36e3c2a1b57f3066
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# WebSocket to container

Forwarding a Websocket request to a Container

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/examples/websocket/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Forward WebSocket upgrade requests through a Durable Object to a Container. This example runs an echo server with the `durable_object` scheduling policy.

## Configure the Worker

```jsonc
{
  "$schema": "./node_modules/wrangler/config-schema.json",
  "name": "websocket-container",
  "main": "src/index.ts",
  "upload_source_maps": true,
  // Set this to today's date
  "compatibility_date": "2026-10-14",
  "observability": {
    "enabled": true
  },
  "containers": [
    {
      "class_name": "MyContainer",
      "scheduling_policy": "durable_object",
      "images": {
        "base": {
          "dockerfile": "./Dockerfile"
        }
      }
    }
  ],
  "durable_objects": {
    "bindings": [
      {
        "name": "MY_CONTAINER",
        "class_name": "MyContainer"
      }
    ]
  },
  "exports": {
    "MyContainer": {
      "type": "durable-object",
      "storage": "sqlite"
    }
  }
}
```

```toml
name = "websocket-container"
main = "src/index.ts"
upload_source_maps = true
# Set this to today's date
compatibility_date = "2026-10-14"

[observability]
enabled = true

[[containers]]
class_name = "MyContainer"
scheduling_policy = "durable_object"

[containers.images.base]
dockerfile = "./Dockerfile"

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[exports.MyContainer]
type = "durable-object"
storage = "sqlite"
```

## Forward the upgrade

`start()` initiates startup. The Worker waits for a successful `/health` response before forwarding traffic. Concurrent requests share that readiness check, and a failed check can be retried by the next request.

Return the port response directly to preserve the WebSocket upgrade.

*src/index.jsjs*

```js
import { DurableObject } from "cloudflare:workers";

const INACTIVITY_TIMEOUT_MS = 2 * 60 * 1000;

export class MyContainer extends DurableObject {
	starting;

	constructor(ctx, env) {
		super(ctx, env);
		const container = ctx.container;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request) {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container.getTcpPort(8080).fetch(forwarded);
	}

	async startAndWaitForPort() {
		const container = this.ctx.container;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request, env) {
		return env.MY_CONTAINER.getByName("demo").fetch(request);
	},
};
```

*src/index.tsts*

```ts
import { DurableObject } from "cloudflare:workers";

interface Env {
	MY_CONTAINER: DurableObjectNamespace<MyContainer>;
}

const INACTIVITY_TIMEOUT_MS = 2 * 60 * 1000;

export class MyContainer extends DurableObject<Env> {
	private starting: Promise<void> | undefined;

	constructor(ctx: DurableObjectState, env: Env) {
		super(ctx, env);
		const container = ctx.container!;
		if (container.running) {
			void ctx.blockConcurrencyWhile(() =>
				container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS),
			);
		}
	}

	async fetch(request: Request): Promise<Response> {
		// Concurrent requests share startup and readiness checks.
		this.starting ??= this.startAndWaitForPort().finally(() => {
			this.starting = undefined;
		});
		await this.starting;

		const url = new URL(request.url);
		url.protocol = "http:";
		url.host = "container";
		const forwarded = new Request(url, request);
		forwarded.headers.delete("host");
		return this.ctx.container!.getTcpPort(8080).fetch(forwarded);
	}

	private async startAndWaitForPort(): Promise<void> {
		const container = this.ctx.container!;
		if (!container.running) {
			container.start({
				image: container.images.base,
				instance: "lite",
				enableInternet: false,
			});
		}
		await container.setInactivityTimeout(INACTIVITY_TIMEOUT_MS);

		const port = container.getTcpPort(8080);
		let lastError: unknown;
		for (let attempt = 0; attempt < 100; attempt++) {
			try {
				const response = await port.fetch("http://container/health", {
					signal: AbortSignal.timeout(1000),
				});
				await response.body?.cancel();
				if (!response.ok) {
					throw new Error(`Health check returned ${response.status}`);
				}
				return;
			} catch (error) {
				lastError = error;
				await scheduler.wait(200);
			}
		}
		throw new Error("Container did not become ready on port 8080", {
			cause: lastError,
		});
	}
}

export default {
	fetch(request: Request, env: Env): Promise<Response> {
		return env.MY_CONTAINER.getByName("demo").fetch(request);
	},
} satisfies ExportedHandler<Env>;
```

## Define the WebSocket server

Save these files in the project root. The HTTP server handles `/health`, while `/ws` accepts WebSocket upgrades.

*package.jsonjson*

```json
{
	"private": true,
	"type": "module",
	"dependencies": { "ws": "^8.18.0" }
}
```

*server.mjsjs*

```js
import { createServer } from "node:http";
import { WebSocketServer } from "ws";

const server = createServer((request, response) => {
	if (request.url === "/health") {
		response.writeHead(200).end();
	} else {
		response.writeHead(404).end();
	}
});
const sockets = new WebSocketServer({ server, path: "/ws" });
sockets.on("connection", (socket) => {
	socket.on("error", console.error);
	socket.on("message", (data, isBinary) => {
		socket.send(data, { binary: isBinary });
	});
});
server.listen(8080, "0.0.0.0");
```

*Dockerfiledockerfile*

```dockerfile
FROM node:24-bookworm-slim
WORKDIR /app
COPY package.json .
RUN npm install --omit=dev
COPY server.mjs .
EXPOSE 8080
CMD ["node", "server.mjs"]
```

## Run the example

Start a Docker-compatible engine. Install Wrangler in your project, using version 4.136.0 or later for [local development](https://developers.cloudflare.com/containers/guides/local-dev/).

npmyarnpnpmbun

```
npm i -D wrangler
```

```
yarn add -D wrangler
```

```
pnpm add -D wrangler
```

```
bun add -d wrangler
```

npmyarnpnpm

```
npx wrangler dev
```

```
yarn wrangler dev
```

```
pnpm wrangler dev
```

Connect a WebSocket client to `ws://localhost:8787/ws`. Send a message and check that the same message returns. For a deployed Worker, use `wss://<WORKER_HOST>/ws`.

An open WebSocket can keep the Durable Object active. The inactivity timeout applies after the Durable Object becomes inactive. Refer to [Container lifecycle](https://developers.cloudflare.com/containers/concepts/architecture/).

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/examples/websocket/#page","headline":"WebSocket to container","description":"Forwarding a Websocket request to a Container","url":"https://developers.cloudflare.com/containers/examples/websocket/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/examples/websocket/og.png?v=36e3c2a1b57f3066","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Lookup Wrangler configuration and commands for Containers.
title: Reference
image: https://developers.cloudflare.com/containers/reference/og.png?v=c5a7efd689e3d7c7
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Reference

Last updated Sep 29, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/reference/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/reference/#page","headline":"Reference","description":"Lookup Wrangler configuration and commands for Containers.","url":"https://developers.cloudflare.com/containers/reference/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/reference/og.png?v=c5a7efd689e3d7c7","dateModified":"2026-09-29","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Wrangler commands for interacting with Cloudflare's Container Platform.
title: Containers
image: https://developers.cloudflare.com/workers/wrangler/commands/containers/og.png?v=9e15e65993753696
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/workers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Containers

Last updated Apr 23, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/workers/wrangler/commands/containers/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Interact with [Containers](https://developers.cloudflare.com/containers/) using Wrangler.

### `build`

Build a Container image from a Dockerfile.

```txt
wrangler containers build [PATH] [OPTIONS]
```

- `PATH` `string` optional
  - Path for the directory containing the Dockerfile to build.
- `-t, --tag` `string` required
  - Name and optionally a tag (format: "name:tag").
- `--path-to-docker` `string` optional
  - Path to your docker binary if it's not on `$PATH`.
  - Default: "docker"
- `-p, --push` `boolean` optional
  - Push the built image to Cloudflare's managed registry.
  - Default: false

### `delete`

Delete a Container (application).

```txt
wrangler containers delete <CONTAINER_ID> [OPTIONS]
```

- `CONTAINER_ID` `string` required
  - The ID of the Container to delete.

### `images`

Perform operations on images in your containers registry.

#### `images list`

List images in your containers registry.

```txt
wrangler containers images list [OPTIONS]
```

- `--filter` `string` optional
  - Regex to filter results.
- `--json` `boolean` optional
  - Return output as clean JSON.
  - Default: false

#### `images delete`

Remove an image from your containers registry.

```txt
wrangler containers images delete [IMAGE] [OPTIONS]
```

- `IMAGE` `string` required
  - Image to delete of the form `IMAGE:TAG`

### `registries`

Configure and view registries available to your container. [Read more](https://developers.cloudflare.com/containers/guides/image-management/#using-amazon-ecr-container-images) about our currently supported external registries.

#### `registries list`

List registries your containers are able to use.

```txt
wrangler containers registries list [OPTIONS]
```

- `--json` `boolean` optional
  - Return output as clean JSON.
  - Default: false

#### `registries configure`

Configure a new registry for your account.

```txt
wrangler containers registries configure [DOMAIN] [OPTIONS]
```

- `DOMAIN` `string` required
  - Domain to configure for the registry.
- `--dockerhub-username` `string` optional
  - The Docker Hub username to authenticate with. Use with the `docker.io` domain. The secret is a Docker Hub personal access token.
- `--aws-access-key-id` `string` optional
  - The AWS access key ID to authenticate with. Use with an Amazon ECR domain. The secret is the matching AWS secret access key.
- `--gar-email` `string` optional
  - The Google service account email to authenticate with. Use with a `*-docker.pkg.dev` domain.
- `--secret-store-id` `string` optional
  - The ID of the secret store to use to store the registry credentials
- `--secret-name` `string` optional
  - The name Wrangler should store the registry credentials under

The credential flags are mutually exclusive. Use the one that matches the registry you are configuring.

When run interactively, wrangler will prompt you for your secret and store it in Secrets Store. To run non-interactively, you can send your secret value to wrangler through stdin to have the secret created for you.

#### `registries delete`

Remove a registry configuration from your account.

```txt
wrangler containers registries delete [DOMAIN] [OPTIONS]
```

- `DOMAIN` `string` required
  - domain of the registry to delete

#### `registries credentials`

Generate temporary credentials to push or pull images from the Cloudflare managed registry (`registry.cloudflare.com`).

```txt
wrangler containers registries credentials [OPTIONS]
```

- `--push` `boolean` optional
  - Generate credentials with push permission.
- `--pull` `boolean` optional
  - Generate credentials with pull permission.
- `--expiration-minutes` `number` optional
  - How long the credentials should be valid for (in minutes).
  - Default: 15

At least one of `--push` or `--pull` must be specified.

### `info`

Get information about a specific Container, including top-level details and a list of instances.

```txt
wrangler containers info <CONTAINER_ID> [OPTIONS]
```

- `CONTAINER_ID` `string` required
  - The ID of the Container to get information about.

### `instances`

List all Container instances for a given application. Displays instance ID, name, state, location, version, and creation time.

In interactive mode, results are paginated. Press `Enter` to load the next page or `Esc`/`q` to stop. In non-interactive environments (for example, when piping output or running in CI), all pages are fetched automatically.

Use the `--json` flag to return output as a flat JSON array. Each element contains the fields `id`, `name`, `state`, `location`, `version`, and `created`. This is also the default output format in non-interactive environments.

```txt
wrangler containers instances <APPLICATION_ID> [OPTIONS]
```

- `APPLICATION_ID` `string` required
  - The UUID of the application to list instances for. Use `wrangler containers list` to find application IDs.
- `--per-page` `number` optional
  - Number of instances per page.
  - Default: 25
- `--json` `boolean` optional
  - Return output as clean JSON.
  - Default: false

For example, to list instances for an application:

```sh
wrangler containers instances 12345678-abcd-1234-abcd-123456789abc
```

```sh
INSTANCE                              NAME        STATE          LOCATION  VERSION  CREATED
a1b2c3d4-e5f6-7890-abcd-ef1234567890  worker-12   running        sfo06     3        2025-06-01T12:00:00Z
b2c3d4e5-f6a7-8901-bcde-f12345678901  worker-47   provisioning   iad01     2        2025-06-01T13:00:00Z
```

To get the same data as JSON:

```sh
wrangler containers instances 12345678-abcd-1234-abcd-123456789abc --json
```

```json
[
	{
		"id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
		"name": "worker-12",
		"state": "running",
		"location": "sfo06",
		"version": 3,
		"created": "2025-06-01T12:00:00Z"
	}
]
```

### `list`

List the Containers in your account.

```txt
wrangler containers list [OPTIONS]
```

### `push`

Push a tagged image to a Cloudflare managed registry, which is automatically integrated with your account.

```txt
wrangler containers push [TAG] [OPTIONS]
```

- `TAG` `string` required
  - The name and tag of the container image to push.
- `--path-to-docker` `string` optional
  - Path to your docker binary if it's not on `$PATH`.
  - Default: "docker"

### `ssh`

Connect to a running Container instance using SSH. Refer to [SSH](https://developers.cloudflare.com/containers/guides/ssh/) for configuration details.

```txt
wrangler containers ssh <INSTANCE_ID> [OPTIONS]
```

You can also specify a command to run, instead of the default shell. For example:

```txt
wrangler containers ssh <INSTANCE_ID> -- ls -al
```

Only the default shell gets a terminal automatically. To run an interactive command, such as `top`, pass `--tty`:

```txt
wrangler containers ssh <INSTANCE_ID> --tty -- top
```

- `INSTANCE_ID` `string` required
  - The ID of the Container instance to SSH into.
- `-t, --tty` `boolean` optional
  - Force pseudo-terminal (PTY) allocation. Use this when running an interactive command.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/workers/wrangler/commands/containers/#page","headline":"Containers","description":"Wrangler commands for interacting with Cloudflare's Container Platform.","url":"https://developers.cloudflare.com/workers/wrangler/commands/containers/","inLanguage":"en","image":"https://developers.cloudflare.com/workers/wrangler/commands/containers/og.png?v=9e15e65993753696","dateModified":"2026-04-23","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Answers to common questions about Containers, including logging, scaling, cold starts, disk persistence, and rollouts.
title: Frequently Asked Questions
image: https://developers.cloudflare.com/containers/faq/og.png?v=54d27de1bf2e71bb
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Frequently Asked Questions

Last updated Oct 5, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/faq/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

## How do Container logs work?

To get logs in the Dashboard, including live tailing of logs, turn on observability in your Wrangler config.

You can turn it on at the Worker level, which applies to containers by default:

```jsonc
{
	"observability": {
		"enabled": true
	}
}
```

```toml
[observability]
enabled = true
```

You can also configure observability per container:

```jsonc
{
	"containers": [
		{
			"class_name": "MyContainer",
			"image": "./Dockerfile",
			"observability": {
				"enabled": true
			}
		}
	]
}
```

```toml
[[containers]]
class_name = "MyContainer"
image = "./Dockerfile"

  [containers.observability]
  enabled = true
```

If a container sets `observability` in its container entry, it overrides the top-level `observability` setting for that container.

Logs are subject to the same [limits as Workers Logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#limits) and are retained for seven days.

Beginning December 1, 2026, Container logs use [Cloudflare Observability pricing](https://developers.cloudflare.com/observability/pricing/).

You can export container logs via [Logpush](https://developers.cloudflare.com/logs/logpush/) to your preferred destination.

## How are container instance locations selected?

When initially deploying a Container, Cloudflare will select various locations across our network to deploy instances to. These locations will span multiple regions.

When a Container instance is requested with `this.ctx.container.start`, the nearest free container instance will be selected from the pre-initialized locations. This will likely be in the same region as the external request, but may not be. Once the container instance is running, any future requests will be routed to the initial location.

An Example:

- A user deploys a Container. Cloudflare automatically readies instances across its Network.
- A request is made from a client in Bariloche, Argentina. It reaches the Worker in Cloudflare's location in Neuquen, Argentina.
- This Worker request calls `MY_CONTAINER.get("session-1337")` which brings up a Durable Object, which then calls `this.ctx.container.start`.
- This requests the nearest free Container instance.
- Cloudflare recognizes that an instance is free in Buenos Aires, Argentina, and starts it there.
- A different user needs to route to the same container. This user's request reaches the Worker running in Cloudflare's location in San Diego.
- The Worker again calls `MY_CONTAINER.get("session-1337")`.
- If the initial container instance is still running, the request is routed to the location in Buenos Aires. If the initial container has gone to sleep, Cloudflare will once again try to find the nearest "free" instance of the Container, likely one in North America, and start an instance there.

## How do container updates and rollouts work?

On `wrangler deploy`, the Worker goes live first. Container instances update with a gradual rollout by default. Refer to [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/) for steps, grace periods, and modes. Refer to [Deploy Containers](https://developers.cloudflare.com/containers/guides/deploy/) to run a deploy.

## How do Workers Builds work with Containers?

On the production branch, Workers Builds should run `wrangler deploy` so images and container instances can update. Non-production Workers Builds defaults to `wrangler versions upload`, which does not update images. Containers Workers implement Durable Objects, so preview URLs are not generated for them. Refer to [Deploy Containers](https://developers.cloudflare.com/containers/guides/deploy/#before-production).

## How does scaling work?

Containers scale by creating or addressing specific instances. For stateless routing across a fixed number of interchangeable instances, use the `getRandom` helper.

Refer to [scaling and routing](https://developers.cloudflare.com/containers/configuration/scaling-and-routing/) for details.

### Is built-in autoscaling for stateless applications available?

Not today, though Cloudflare plans to add built-in autoscaling in a future release.

Until then, use `getRandom` for simple stateless routing and specific instance IDs when you need explicit control over container lifecycle.

## What are cold starts? How fast are they?

A cold start is when a container instance is started from a completely stopped state.

If you call `env.MY_CONTAINER.get(id)` with a completely novel ID and launch this instance for the first time, it will result in a cold start.

This will start the container image from its entrypoint for the first time. Depending on what this entrypoint does, it will take a variable amount of time to start.

Container cold starts can often be in the 1-3 second range, but this is dependent on image size and code execution time, among other factors.

## How do I use an existing container image?

Refer to [image management](https://developers.cloudflare.com/containers/guides/image-management/#use-pre-built-container-images).

## Is disk persistent? What happens to my disk when my container sleeps?

All disk is ephemeral by default. When a Container instance goes to sleep, the next time it starts, it uses a fresh disk from the container image.

If you need point-in-time filesystem state, Container applications that use the [`durable_object` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#use-the-durable-object-scheduling-policy) can create and restore a snapshot. Snapshots are immutable, so later file changes require a new snapshot. For more information, refer to [Snapshots](https://developers.cloudflare.com/containers/guides/snapshots/).

You can also use [FUSE](https://developers.cloudflare.com/containers/examples/r2-fuse-mount/) to persist disk to R2 or other object storage backends. Though you should not expect native SSD-like performance while using FUSE.

## What happens if I run out of memory?

If you run out of memory, your instance will throw an Out of Memory (OOM) error and will be restarted.

Containers do not use swap memory.

## How long can instances run for? What happens when a host server is shut down?

Cloudflare does not stop a container instance after a fixed maximum runtime. With the Durable Object Container API, call [`setInactivityTimeout()`](https://developers.cloudflare.com/containers/api/durable-object-container/#setinactivitytimeout) to stop an inactive container. The `Container` class sets [`sleepAfter`](https://developers.cloudflare.com/containers/api/container-class/#sleepafter) to 10 minutes by default. Its [`onActivityExpired()`](https://developers.cloudflare.com/containers/api/container-class/#onactivityexpired) implementation signals the container to stop after that period without activity. You can change the duration or override the hook.

Another platform event can stop an active container. For example, a host server restart happens on an irregular cadence. Cloudflare does not guarantee that any container instance will run for a set period.

When the platform is about to stop a container instance (including before a host moves work off a server), it:

1. Sends `SIGTERM` to the main process in the container.
2. Waits up to 15 minutes for that process to exit.
3. Sends `SIGKILL` if the process is still running.

Handle `SIGTERM` in your image if you need cleanup before exit. After a host stop, a new container instance may start on a different server when traffic needs it again.

Image updates during a deploy use the same stop sequence. Refer to [Rollouts](https://developers.cloudflare.com/containers/configuration/rollouts/).

## How can I pass secrets to my container?

You can use [Worker Secrets](https://developers.cloudflare.com/workers/configuration/secrets/) or the [Secrets Store](https://developers.cloudflare.com/secrets-store/integrations/workers/) to define secrets for your Workers.

For implementation details, refer to [Environment variables and secrets](https://developers.cloudflare.com/containers/examples/env-vars-and-secrets/).

## Can I run Docker inside a container (Docker-in-Docker)?

Yes. Use the `docker:dind` image, and start the Docker daemon with iptables and IP forwarding turned off:

*Dockerfiledockerfile*

```dockerfile
FROM docker:dind

# Start dockerd with iptables and IP forwarding turned off, then run your app
ENTRYPOINT ["sh", "-c", "dockerd-entrypoint.sh dockerd --iptables=false --ip6tables=false --ip-forward=false & exec /path/to/your-app"]
```

This Dockerfile works with both [scheduling policies](https://developers.cloudflare.com/containers/configuration/scheduling-policy/). Containers that use the `durable_object` policy cannot turn on IP forwarding. In those containers, the Docker daemon exits on startup unless you pass `--ip-forward=false`.

Run the Docker daemon as `root`. Rootless Docker does not start in Containers.

If your application needs to wait for dockerd to become ready before using Docker, use an entrypoint script instead of the inline command above:

*entrypoint.shsh*

```sh
#!/bin/sh
set -eu

# Wait for dockerd to be ready
until docker version >/dev/null 2>&1; do
  sleep 0.2
done

exec /path/to/your-app
```

Working with disabled iptables

Cloudflare Containers do not support iptables manipulation. The `--iptables=false` and `--ip6tables=false` flags prevent Docker from attempting to configure network rules, which would otherwise fail.

To send or receive traffic from a container running within Docker-in-Docker, use the `--network=host` flag with `docker run`. A `docker build` step that uses the network, such as a package install, also needs `docker build --network=host`.

This allows you to connect to the container, but it means each inner container has access to your outer container's network stack. Ensure you understand the security implications of this setup before proceeding.

For a complete working example, refer to the [Docker-in-Docker Containers example ↗︎](https://github.com/th0m/containers-dind). The example uses the `default` scheduling policy. To run it with `scheduling_policy: "durable_object"`, add `--ip-forward=false` to its `dockerd` flags.

## How do I allow or disallow egress from my container?

Refer to [Handle outbound traffic](https://developers.cloudflare.com/containers/configuration/outbound-traffic/) for how to control outbound traffic and internet access.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/faq/#page","headline":"Frequently Asked Questions","description":"Answers to common questions about Containers, including logging, scaling, cold starts, disk persistence, and rollouts.","url":"https://developers.cloudflare.com/containers/faq/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/faq/og.png?v=54d27de1bf2e71bb","dateModified":"2026-10-05","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Product-wide information for Containers, including pricing and limits.
title: Platform
image: https://developers.cloudflare.com/containers/platform/og.png?v=d1bf5f78659cbac8
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Platform

Last updated Sep 29, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/platform/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/containers/platform/#page","headline":"Platform","description":"Product-wide information for Containers, including pricing and limits.","url":"https://developers.cloudflare.com/containers/platform/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/platform/og.png?v=d1bf5f78659cbac8","dateModified":"2026-09-29","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Available Container instance types and limits for memory, vCPU, disk, image storage, and snapshots.
title: Limits and Instance Types
image: https://developers.cloudflare.com/containers/platform/limits/og.png?v=976edb39343c58ee
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Limits and Instance Types

Last updated Oct 12, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/platform/limits/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

Note

All instance size maximums and account limits on this page are defaults and can be increased. To request higher limits, contact your account team, file a support ticket, or fill out the [limit increase request form ↗︎](https://forms.gle/CscdaEGuw5Hb6H2s7).

## Instance Types

The memory, vCPU, and disk space for Containers are set through instance types. You can use one of six predefined instance types or configure a [custom instance type](#custom-instance-types).

| Instance Type | vCPU | Memory | Disk |
| --- | --- | --- | --- |
| lite | 1/16 | 256 MiB | 2 GB |
| basic | 1/4 | 1 GiB | 4 GB |
| standard-1 | 1/2 | 4 GiB | 8 GB |
| standard-2 | 1 | 6 GiB | 12 GB |
| standard-3 | 2 | 8 GiB | 16 GB |
| standard-4 | 4 | 12 GiB | 20 GB |

For an application that uses the [`default` scheduling policy](https://developers.cloudflare.com/containers/configuration/scheduling-policy/), specify the size with the [`instance_type` property](https://developers.cloudflare.com/workers/wrangler/configuration/#containers) in your Worker's Wrangler configuration file. For the `durable_object` policy, pass the named size to `ctx.container.start()` with the runtime `instance` property.

Note

The `dev` and `standard` instance types are preserved for backward compatibility and are aliases for `lite` and `standard-1`, respectively.

### Custom Instance Types

In addition to the predefined instance types, you can configure custom instance types. Field names depend on where you configure the size. Wrangler configuration for the `default` policy uses `vcpu`, `memory_mib`, and `disk_mb`. A `ctx.container.start()` call for the `durable_object` policy uses `vcpu`, `memoryMib`, and `diskMb`.

Refer to the [Wrangler configuration documentation](https://developers.cloudflare.com/workers/wrangler/configuration/#custom-instance-types) or [scheduling policy documentation](https://developers.cloudflare.com/containers/configuration/scheduling-policy/#choose-an-instance-size-at-runtime) for examples.

Custom instance types have the following constraints:

| Resource | Limit |
| --- | --- |
| Minimum vCPU | 1 |
| Maximum vCPU | 4 |
| Maximum Memory | 12 GiB |
| Maximum Disk | 20 GB |
| Memory to vCPU ratio | Minimum 3 GiB memory per vCPU |

For workloads requiring less than 1 vCPU, use the predefined instance types such as `lite` or `basic`.

## Account limits

The following limits apply per account:

| Resource | Limit |
| --- | --- |
| Concurrent memory | 6 TiB |
| Concurrent vCPU | 1,500 |
| Concurrent disk | 30 TB |
| Image size | Same as [instance disk space](#instance-types) |
| Total image storage per account | 50 GB <sup>[1](#user-content-fn-1)</sup> |

## Snapshot limits

The following limits apply to [Container snapshots](https://developers.cloudflare.com/containers/guides/snapshots/):

| Resource | Limit |
| --- | --- |
| Maximum snapshot size | 20 GB |
| Snapshot retention | 30 days from creation or the most recent restore |

Restoring a snapshot refreshes its 30-day time-to-live.

## Footnotes

1. Delete container images with `wrangler containers delete` to free up space. If you delete a container image and then [roll back](https://developers.cloudflare.com/workers/versions-and-deployments/rollbacks/) your Worker to a previous version, this version may no longer work. [↩](#user-content-fnref-1)

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/platform/limits/#page","headline":"Limits and Instance Types","description":"Available Container instance types and limits for memory, vCPU, disk, image storage, and snapshots.","url":"https://developers.cloudflare.com/containers/platform/limits/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/platform/limits/og.png?v=976edb39343c58ee","dateModified":"2026-10-12","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

---

---
description: Billing rates for Containers vCPU, memory, disk, and network egress, including included usage on the Workers Paid plan.
title: Pricing
image: https://developers.cloudflare.com/containers/platform/pricing/og.png?v=6cefb4d67718aa46
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/containers/llms.txt  
> Use this file to discover all available pages before exploring further.

# Pricing

Last updated Oct 5, 2026|Copy as Markdown| [View as Markdown](https://developers.cloudflare.com/containers/platform/pricing/index.md)| [Agent setup](https://developers.cloudflare.com/agent-setup/)

## vCPU, Memory and Disk

Containers are billed for every 10ms that they are actively running at the following rates, with included monthly usage as part of the $5 USD per month [Workers Paid plan](https://developers.cloudflare.com/workers/platform/pricing/):

|  | Memory | CPU | Disk |
| --- | --- | --- | --- |
| **Free** | N/A | N/A | |
| **Workers Paid** | 25 GiB-hours/month included <br> +$0.0000025 per additional GiB-second | 375 vCPU-minutes/month <br>+ $0.000020 per additional vCPU-second | 200 GB-hours/month <br> +$0.00000007 per additional GB-second |

You only pay for what you use — charges start when a request is sent to the container or when it is manually started. Charges stop after the container instance goes to sleep, which can happen automatically after a timeout. This makes it easy to scale to zero, and allows you to get high utilization even with bursty traffic.

Memory and disk usage are based on the *provisioned resources* for the instance type you select, while CPU usage is based on *active usage* only.

#### Instance Types

When you deploy a container, you specify an [instance type](https://developers.cloudflare.com/containers/platform/limits/#instance-types).

The instance type you select will impact your bill — larger instances include more memory and disk, incurring additional costs, and higher CPU capacity, which allows you to incur higher CPU costs based on active usage.

The following instance types are currently available:

| Instance Type | vCPU | Memory | Disk |
| --- | --- | --- | --- |
| lite | 1/16 | 256 MiB | 2 GB |
| basic | 1/4 | 1 GiB | 4 GB |
| standard-1 | 1/2 | 4 GiB | 8 GB |
| standard-2 | 1 | 6 GiB | 12 GB |
| standard-3 | 2 | 8 GiB | 16 GB |
| standard-4 | 4 | 12 GiB | 20 GB |

## Network Egress

Egress from Containers is priced at the following rates:

| Region | Price per GB | Included Allotment per month |
| --- | --- | --- |
| North America & Europe | $0.025 | 1 TB |
| Oceania, Korea, Taiwan | $0.05 | 500 GB |
| Everywhere Else | $0.04 | 500 GB |

## Workers and Durable Objects Pricing

When you use Containers, incoming requests to your containers are handled by your [Worker](https://developers.cloudflare.com/workers/platform/pricing/), and each container has its own [Durable Object](https://developers.cloudflare.com/durable-objects/platform/pricing/). You are billed for your usage of both Workers and Durable Objects.

## Logs and Observability

Containers are integrated with the [Workers Logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/) platform, and billed at the same rate. Refer to [Workers Logs pricing](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#pricing) for details.

When you [turn on observability for your Worker](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#enable-workers-logs), or configure `observability` on a specific container, logs from that container will show in both the Containers and Observability sections of the Cloudflare dashboard.

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/containers/platform/pricing/#page","headline":"Pricing","description":"Billing rates for Containers vCPU, memory, disk, and network egress, including included usage on the Workers Paid plan.","url":"https://developers.cloudflare.com/containers/platform/pricing/","inLanguage":"en","image":"https://developers.cloudflare.com/containers/platform/pricing/og.png?v=6cefb4d67718aa46","dateModified":"2026-10-05","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```

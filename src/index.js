import { DurableObject } from "cloudflare:workers";

export class ChecksumContainer extends DurableObject {
	async fetch(request) {
		const c = this.ctx.container;
		await c.setInactivityTimeout(30 * 1000);
		if (!c.running) c.start();
		const port = c.getTcpPort(4000);
		// start() returns before the container is ready, so poll /health.
		for (let i = 0; i < 100; i++) {
			try {
				if ((await port.fetch("http://container/health")).ok) break;
			} catch {}
			await scheduler.wait(200);
		}
		const url = new URL(request.url);
		return port.fetch(new Request(`http://container${url.pathname}${url.search}`, request));
	}
}

export default {
	// ponytail: one shared instance; use getByName(id) per key to scale out
	fetch: (request, env) => env.CHECKSUM.getByName("main").fetch(request),
};

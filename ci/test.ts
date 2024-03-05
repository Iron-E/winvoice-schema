import './scope';
import { Container } from '@dagger.io/dagger';
import { enqueue, inject } from '@iron-e/scabbard';

enqueue('tests', async _ => {
	const withCargo = inject('withCargo').instance(Container);
	const output = await withCargo
		.pipeline('run')
		.withExec(["cargo", "hack", "--feature-powerset", "test"])
		.stdout();

	console.log(output);
});

await import.meta.filename.runPipelinesIfMain();

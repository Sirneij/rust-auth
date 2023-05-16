<script lang="ts">
	import ActixWebSvelte from '$lib/svgs/rust.jpg';
	import DjangoSvelte from '$lib/svgs/django.jpg';
	import { actixTopics } from '$lib/utils/constant';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import type { Topic } from '$lib/utils/types';
	import { flip } from 'svelte/animate';

	let someactixTopics = [actixTopics[0]],
		next = 1;

	const addTopic = () => {
		if (next < actixTopics.length) {
			someactixTopics.push(actixTopics[next]);
			someactixTopics = someactixTopics;
			next += 1;
		}
	};

	function removeTopic(topic: Topic) {
		someactixTopics = someactixTopics.filter((t) => t !== topic);
		next -= 1;
	}
</script>

<svelte:head>
	<title>Written articles | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="hero-container">
	<div class="hero fade">
		<img src={ActixWebSvelte} alt="" />
		<div class="hero-text">
			<h1 style="font-size:50px">I am John Doe</h1>
			<p>And I'm a Photographer</p>
			<button>Hire me</button>
		</div>
	</div>

	<div class="hero fade">
		<img src={DjangoSvelte} alt="" />
		<div class="hero-text">
			<h1 style="font-size:50px">I am John Doe</h1>
			<p>And I'm a Photographer</p>
			<button>Hire me</button>
		</div>
	</div>
</div>

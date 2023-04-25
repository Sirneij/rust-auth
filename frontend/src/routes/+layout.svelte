<script lang="ts">
	import { navigating } from '$app/stores';
	import Header from '$lib/component/Header/Header.svelte';
	import Loader from '$lib/component/Loader/Loader.svelte';
	import Notification from '$lib/component/Notification/Notification.svelte';
	import PageTransition from '$lib/component/Transition/PageTransition.svelte';
	import '$lib/css/app.css';
	import { loading } from '$lib/stores/loading.store';
	import type { PageData } from './$types';

	export let data: PageData;

	$: loading.setNavigate(!!$navigating);
	$: loading.setLoading(!!$navigating, 'Loading, please wait...');
</script>

<PageTransition key={data.url} duration={600}>
	<Header />
	<Notification />
	<Loader />
	<slot />
</PageTransition>

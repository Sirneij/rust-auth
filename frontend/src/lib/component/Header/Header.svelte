<script lang="ts">
	import JohnImage from '$lib/svgs/john.svg';
</script>

<header aria-label="Page Header" class="mb-6">
	<nav class="mx-auto max-w-screen-xl px-4 py-4 sm:px-6 lg:px-8">
		<div class="flex items-center justify-end gap-4">
			<div class="flex items-center gap-4">
				<a
					href="https://github.com/Sirneij"
					class="block shrink-0 rounded-full bg-white p-2.5 text-gray-600 shadow-sm hover:text-gray-700"
				>
					<span class="sr-only">Programmer</span>
					<img src={JohnImage} alt="John Idogun" class="h-6 w-6 rounded-full object-cover" />
				</a>
			</div>

			<span aria-hidden="true" class="block h-6 w-px rounded-full bg-gray-200" />

			<a href="/" class="block shrink-0 text-white hover:text-sky-400">/</a>

			<a href="/auth/login" class="block shrink-0 text-white hover:text-sky-400">Login</a>

			<a href={null} class="block shrink-0">
				<span class="sr-only">Profile</span>
				<img
					alt="Man"
					src="https://images.unsplash.com/photo-1600486913747-55e5470d6f40?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
					class="h-10 w-10 rounded-full object-cover"
				/>
			</a>
			<button type="button" class="text-white hover:text-sky-400">Logout</button>
		</div>
	</nav>
</header>

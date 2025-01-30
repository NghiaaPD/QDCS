<!-- <script lang="ts">
	import { goto } from '$app/navigation';

	interface Subject {
		value: string;
		label: string;
	}

	const subjects: Subject[] = [
		{ value: 'NWC', label: 'NWC' },
		{ value: 'CPV', label: 'CPV' },
		{ value: 'CSD', label: 'CSD' },
		{ value: 'SWE', label: 'SWE' },
		{ value: 'SWP', label: 'SWP' }
	];

	let searchTerm = '';
	let showDropdown = false;
	let selectedSubject = '';
	let selectedValue = '';

	$: filteredSubjects = subjects.filter((subject) =>
		subject.label.toLowerCase().includes(searchTerm.toLowerCase())
	);

	function selectSubject(subject: Subject) {
		selectedSubject = subject.value;
		selectedValue = subject.value;
		searchTerm = subject.label;
		showDropdown = false;
	}

	async function handleSubmit() {
		await goto('/check');
	}
</script>

<div class="area">
	<ul class="circles">
		<li></li>
		<li></li>
		<li></li>
		<li></li>
		<li></li>
		<li></li>
		<li></li>
		<li></li>
		<li></li>
		<li></li>
	</ul>
</div>

<div class="flex min-h-screen items-center justify-center">
	<div
		class="w-96 animate-[zoomIn_0.6s_ease-in-out] rounded-lg border border-white/20 bg-white/90 p-8 shadow-2xl backdrop-blur-sm transition-all duration-300 hover:shadow-blue-500/10"
	>
		<div class="flex justify-center">
			<enhanced:img
				src="/static/fptulogo.png"
				alt="FPT Logo"
				class="h-48 w-auto transition-transform duration-300 hover:scale-105"
			/>
		</div>
		<p class="mb-4 text-center text-lg font-medium text-gray-700">Vui lòng đăng nhập để tiếp tục</p>
		<form class="space-y-6" on:submit|preventDefault={handleSubmit}>
			<div class="group">
				<label
					for="staff_code"
					class="block text-sm font-medium text-gray-700 transition-colors duration-200 group-hover:text-blue-600"
					>Mã nhân viên</label
				>
				<input
					type="text"
					id="staff_code"
					class="mt-1 block w-full rounded-md border border-gray-300 bg-white/80 px-3 py-2 text-gray-900 placeholder-gray-400 transition-all duration-200 hover:border-blue-400 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
					placeholder="00111111"
					required
				/>
			</div>
			<div class="group">
				<label
					for="subject"
					class="block text-sm font-medium text-gray-700 transition-colors duration-200 group-hover:text-blue-600"
					>Môn học</label
				>
				<div class="relative">
					<input
						type="text"
						id="subject"
						bind:value={searchTerm}
						on:focus={() => (showDropdown = true)}
						class="mt-1 block w-full rounded-md border border-gray-300 bg-white/80 px-3 py-2 text-gray-900 placeholder-gray-400 transition-all duration-200 hover:border-blue-400 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
						placeholder="Tìm môn học..."
						required
					/>
					{#if showDropdown}
						<div
							role="listbox"
							tabindex="0"
							class="absolute z-10 mt-1 w-full rounded-md border border-gray-200 bg-white/90 shadow-lg backdrop-blur-sm"
							on:mouseleave={() => (showDropdown = false)}
						>
							<ul class="max-h-60 overflow-auto py-1">
								{#each filteredSubjects as subject}
									<li
										role="option"
										aria-selected={subject.value === selectedValue}
										class="cursor-pointer px-3 py-2 transition-colors duration-200 hover:bg-blue-50"
										on:click={() => selectSubject(subject)}
										on:keydown={(e) => e.key === 'Enter' && selectSubject(subject)}
									>
										{subject.label}
									</li>
								{/each}
							</ul>
						</div>
					{/if}
				</div>
			</div>

			<div class="flex items-center justify-between">
				<div class="flex items-center">
					<input
						id="remember-me"
						type="checkbox"
						class="h-4 w-4 rounded border-gray-300 text-blue-600 transition-colors duration-200 focus:ring-blue-500"
					/>
					<label
						for="remember-me"
						class="ml-2 block text-sm text-gray-700 transition-colors duration-200 hover:text-blue-600"
						>Ghi nhớ đăng nhập</label
					>
				</div>
			</div>
			<button
				type="submit"
				class="flex w-full justify-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition-all duration-300 hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/30 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
			>
				Đăng nhập
			</button>
		</form>
		<p class="mt-4 text-center text-sm text-gray-600">
			Vấn đề khi đăng nhập?
			<a
				href="https://www.facebook.com/hoadnt"
				class="font-medium text-blue-600 transition-colors duration-200 hover:text-blue-700"
				>Ấn vào đây</a
			>
		</p>
	</div>
</div> -->

<script lang="ts">
	import CheckPage from './check/+page.svelte';
</script>

<CheckPage />

<script lang="ts">
	import { goto } from '$app/navigation';
	import { dialog } from '@tauri-apps/api';

	interface Subject {
		value: string;
		label: string;
	}

	const subjects: Subject[] = [
		{ value: 'SSL101c', label: 'SSL101c' },
		{ value: 'CPV301', label: 'CPV301' },
		{ value: 'CSD203', label: 'CSD203' },
		{ value: 'SWE201c', label: 'SWE201c' },
		{ value: 'DPL302m', label: 'DPL302m' }
	];

	let searchTerm = '';
	let showDropdown = false;
	let selectedSubject = '';

	$: filteredSubjects = subjects.filter((subject) =>
		subject.label.toLowerCase().includes(searchTerm.toLowerCase())
	);

	function selectSubject(subject: Subject) {
		searchTerm = subject.label;
		selectedSubject = subject.value;
		showDropdown = false;
	}

	async function handleSubmit() {
		try {
			// Kiểm tra có nhập nội dung không
			if (!searchTerm || searchTerm.trim() === '') {
				await dialog.message('Vui lòng nhập hoặc chọn một môn học');
				return;
			}

			// Xác định môn học cuối cùng
			let finalSubject = '';

			// Kiểm tra xem có phải là môn học trong danh sách không
			const foundSubject = subjects.find((s) => s.label.toLowerCase() === searchTerm.toLowerCase());

			if (foundSubject) {
				// Đã chọn từ danh sách
				finalSubject = foundSubject.value;
			} else {
				// Nhập tùy chỉnh - sử dụng nội dung đã nhập
				finalSubject = searchTerm.trim();
			}

			// Log để debug
			console.log('Môn học cuối cùng:', finalSubject);

			// Lưu và chuyển trang
			localStorage.setItem('selectedSubject', finalSubject);
			await goto('/check');
		} catch (error) {
			console.error('Lỗi khi submit:', error);
			await dialog.message('Đã xảy ra lỗi: ' + error);
		}
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
		<p class="mb-4 text-center text-lg font-medium text-gray-700">
			Vui lòng chọn hoặc nhập tên môn học để tiếp tục
		</p>
		<form on:submit|preventDefault={handleSubmit} class="space-y-6">
			<div class="group">
				<label
					for="subject"
					class="block text-sm font-medium text-gray-700 transition-colors duration-200 group-hover:text-[#8E7FDD]"
					>Môn học</label
				>
				<div class="relative">
					<input
						type="text"
						id="subject"
						bind:value={searchTerm}
						on:focus={() => (showDropdown = true)}
						class="mt-1 block w-full rounded-md border border-gray-300 bg-white/80 px-3 py-2 text-gray-900 placeholder-gray-400 transition-all duration-200 hover:border-blue-400 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
						placeholder="Nhập hoặc tìm môn học..."
						required
					/>
					{#if showDropdown && filteredSubjects.length > 0}
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
										aria-selected={searchTerm === subject.label}
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

			<button
				type="submit"
				class="flex w-full justify-center rounded-md border border-transparent bg-[#8E7FDD] px-4 py-2 text-sm font-medium text-white shadow-sm transition-all duration-300 hover:bg-[#4E54C8] hover:shadow-lg hover:shadow-blue-500/30 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
			>
				Tiếp tục
			</button>
		</form>
	</div>
</div>

<!-- <script lang="ts">
	import CheckPage from './check/+page.svelte';
</script>

<CheckPage /> -->

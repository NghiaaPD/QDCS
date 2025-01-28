<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri';
	import { writeBinaryFile } from '@tauri-apps/api/fs';
	import { tempdir } from '@tauri-apps/api/os';

	let fileInput: HTMLInputElement;
	let isUploading = false;
	let progress = 0;
	let showSuccess = false;
	let dragOver = false;
	let uploadedFile: File | null = null;
	let showError = false;
	let errorMessage = '';

	function handleFileSelect(event: Event) {
		const files = (event.target as HTMLInputElement).files;
		if (files?.length) uploadFile(files[0]);
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		dragOver = false;
		const files = event.dataTransfer?.files;
		if (files?.length) uploadFile(files[0]);
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		dragOver = true;
	}

	function handleDragLeave() {
		dragOver = false;
	}

	async function uploadFile(file: File) {
		const allowedExtensions = ['.docx'];
		const fileExtension = file.name.substring(file.name.lastIndexOf('.')).toLowerCase();

		if (!allowedExtensions.includes(fileExtension)) {
			errorMessage = 'Chỉ chấp nhận file .docx. Vui lòng chọn file khác!';
			showError = true;
			if (fileInput) fileInput.value = '';
			setTimeout(() => {
				showError = false;
			}, 3000);
			return;
		}

		isUploading = true;
		progress = 0;

		// Giả lập upload file
		for (let i = 0; i <= 100; i += 10) {
			progress = i;
			await new Promise((resolve) => setTimeout(resolve, 200));
		}

		uploadedFile = file;
		isUploading = false;

		setTimeout(() => {
			showSuccess = false;
			showError = false;
		}, 3000);
	}

	async function processFile() {
		if (!uploadedFile) return;
		const file = uploadedFile;

		try {
			const arrayBuffer = await file.arrayBuffer();
			const uint8Array = new Uint8Array(arrayBuffer);
			const tempDir = await tempdir();
			const tempPath = `${tempDir}/temp_${Date.now()}.docx`;
			await writeBinaryFile(tempPath, uint8Array);

			const result = await invoke('process_docx', {
				filePath: tempPath
			});

			// Parse kết quả
			const data = JSON.parse(result as string);

			// In kết quả câu hỏi
			console.log('=== DANH SÁCH CÂU HỎI ===');
			data.questions.forEach((q: any) => {
				console.log(`Câu hỏi ${q.id}: ${q.text}`);
				console.log('Các đáp án:');
				q.answers.forEach((a: string) => console.log(`- ${a}`));
				console.log(`Đáp án đúng: ${q.correct_answer}`);
				console.log('-------------------');
			});

			// In kết quả embeddings
			console.log('\n=== EMBEDDINGS TỪ DATABASE ===');
			console.log(`Tổng số cặp embedding: ${data.embeddings.length}`);

			// In 2 cặp đầu tiên
			data.embeddings.slice(0, 2).forEach((pair: any, i: number) => {
				const [question, answer] = pair;
				console.log(`\n=== Cặp embedding thứ ${i + 1} ===`);
				console.log(`Question embedding (${question.length} chiều):`, question);
				console.log(`Answer embedding (${answer.length} chiều):`, answer);
			});

			showSuccess = true;
		} catch (error) {
			errorMessage = error as string;
			showError = true;
			console.error('Lỗi:', error);
		}
	}

	function removeFile() {
		uploadedFile = null;
		fileInput.value = '';
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
		class="w-[800px] animate-[zoomIn_0.6s_ease-in-out] rounded-lg border border-white/20 bg-white/90 p-16 shadow-2xl backdrop-blur-sm transition-all duration-300 hover:shadow-blue-500/10"
	>
		<div
			role="button"
			tabindex="0"
			class="flex h-full w-full flex-col items-center justify-center rounded-md border-2 border-dashed p-10 text-center transition-colors duration-300"
			class:border-[#8E7FDD]={dragOver}
			style="border-color: #CCABD6"
			on:drop={handleDrop}
			on:dragover={handleDragOver}
			on:dragleave={handleDragLeave}
			on:click={() => !uploadedFile && fileInput.click()}
			on:keydown={(e) => e.key === 'Enter' && !uploadedFile && fileInput.click()}
		>
			{#if isUploading}
				<div class="w-full max-w-md" transition:fade>
					<div class="mb-4 text-gray-600">Đang tải lên... {progress}%</div>
					<div class="h-2 w-full rounded-full bg-gray-200">
						<div
							class="h-2 rounded-full bg-[#8E7FDD] transition-all duration-300"
							style="width: {progress}%"
						></div>
					</div>
				</div>
			{:else if uploadedFile}
				<div class="flex w-full items-center justify-between rounded-lg border border-gray-300 p-4">
					<div class="flex items-center space-x-4">
						<Icon icon="icon-park-outline:file-doc" class="h-10 w-10 text-[#8E7FDD]" />
						<div class="text-left">
							<p class="text-sm font-medium text-gray-700">{uploadedFile.name}</p>
							<p class="text-xs text-gray-500">{Math.round(uploadedFile.size / 1024)} KB</p>
						</div>
					</div>
					<button
						class="rounded-full bg-gradient-to-r from-red-400 to-red-500 p-2 text-white transition-all duration-300 hover:from-red-500 hover:to-red-600 hover:shadow-md hover:shadow-red-500/30"
						on:click|stopPropagation={removeFile}
					>
						<Icon icon="icon-park-outline:close" class="h-5 w-5" />
					</button>
				</div>
			{:else}
				<Icon
					icon="icon-park-outline:upload-one"
					style="width: 80px; height: 80px; color: #8E7FDD"
				/>
				<div class="mt-4 space-y-2 text-gray-600">
					<p>
						Drag & drop <span class="text-[#8E7FDD]">Docx file</span>
					</p>
					<p class="text-sm">
						or <span class="cursor-pointer text-[#8E7FDD] underline">browse files</span> on your computer
					</p>
				</div>
			{/if}
		</div>

		{#if uploadedFile}
			<div class="mt-6">
				<button
					class="w-full rounded-lg bg-gradient-to-r from-[#8E7FDD] to-[#CCABD6] py-3 font-medium text-white transition-all duration-300 hover:shadow-lg hover:shadow-[#8E7FDD]/30"
					on:click={processFile}
				>
					Kiểm tra
				</button>
			</div>
		{/if}
	</div>
</div>

{#if showSuccess}
	<div
		class="fixed bottom-4 right-4 flex items-center gap-3 rounded-lg bg-gradient-to-r from-[#8E7FDD] to-[#CCABD6] px-6 py-4 text-white shadow-xl backdrop-blur-sm transition-all duration-300 hover:shadow-[#8E7FDD]/30"
		transition:fade={{ duration: 300 }}
	>
		<div class="flex h-8 w-8 items-center justify-center rounded-full bg-white/20">
			<Icon icon="icon-park-outline:success" class="h-5 w-5" />
		</div>
		<div class="flex flex-col">
			<span class="font-medium">Thành công!</span>
			<span class="text-sm text-white/80">Đã hoàn thành kiểm tra trùng lặp câu hỏi</span>
		</div>
	</div>
{/if}

{#if showError}
	<div
		class="fixed bottom-4 right-4 flex items-center gap-3 rounded-lg bg-gradient-to-r from-red-500 to-red-600 px-6 py-4 text-white shadow-xl backdrop-blur-sm transition-all duration-300 hover:shadow-red-500/30"
		transition:fade={{ duration: 300 }}
	>
		<div class="flex h-8 w-8 items-center justify-center rounded-full bg-white/20">
			<Icon icon="icon-park-outline:close" class="h-5 w-5" />
		</div>
		<div class="flex flex-col">
			<span class="font-medium">Lỗi!</span>
			<span class="text-sm text-white/80">{errorMessage}</span>
		</div>
	</div>
{/if}

<input
	type="file"
	accept=".docx"
	class="hidden"
	bind:this={fileInput}
	on:change={handleFileSelect}
/>

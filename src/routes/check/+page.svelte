<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri';
	import { writeBinaryFile, readTextFile } from '@tauri-apps/api/fs';
	import { tempdir } from '@tauri-apps/api/os';

	let fileInput: HTMLInputElement;
	let isUploading = false;
	let progress = 0;
	let showSuccess = false;
	let dragOver = false;
	let uploadedFile: File | null = null;
	let showError = false;
	let errorMessage = '';
	let isProcessing = false;
	let showResults = false;
	let similarities: any[] = [];
	let processedFilePath = '';
	let successType: 'check' | 'export' = 'check';
	let isExporting = false;

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

	function adjustSimilarityScore(score: number): number {
		const absoluteScore = Math.abs(score);

		if (absoluteScore >= 1) {
			const randomDeduction = 0.04 + Math.random() * 0.02;
			return 1 - randomDeduction;
		}
		return absoluteScore;
	}

	async function processFile() {
		if (!uploadedFile) return;
		const file = uploadedFile;

		isProcessing = true;
		try {
			const arrayBuffer = await file.arrayBuffer();
			const uint8Array = new Uint8Array(arrayBuffer);
			const tempDir = await tempdir();
			const tempPath = `${tempDir}/temp_${Date.now()}.docx`;
			await writeBinaryFile(tempPath, uint8Array);

			processedFilePath = tempPath;

			const result = await invoke<string>('process_docx', {
				filePath: tempPath
			});

			const parsedResult = JSON.parse(result);

			// Tạo Map để lưu kết quả có độ trùng cao nhất cho mỗi ID
			const bestResults = new Map();

			// Duyệt qua tất cả kết quả để tìm độ trùng cao nhất cho mỗi ID
			parsedResult.similarities.forEach((item: any) => {
				const currentScore = parseFloat(item.similarity_score);
				const existingResult = bestResults.get(item.id);

				if (!existingResult || parseFloat(existingResult.similarity_score) < currentScore) {
					bestResults.set(item.id, item);
				}
			});

			// Chuyển Map thành mảng kết quả cuối cùng
			similarities = Array.from(bestResults.values()).map((item: any) => ({
				id: item.id,
				docx_question: item.docx_question,
				docx_answer: item.docx_answer,
				similarity_score: (adjustSimilarityScore(item.similarity_score) * 100).toFixed(2) + '%',
				is_similar: item.is_similar,
				answers: item.answers,
				true_answer: item.true_answer,
				similar_docx_question: item.similar_docx_question,
				similar_docx_answer: item.similar_docx_answer,
				similar_answers: item.similar_answers,
				similar_true_answer: item.similar_true_answer,
				db_question: item.db_question,
				db_answer: item.db_answer,
				duplicate_type: item.duplicate_type,
				correct_answer_keys: item.correct_answer_keys
			}));

			showSuccess = true;
			successType = 'check';
			showResults = true;

			removeFile();

			setTimeout(() => {
				document.getElementById('results-section')?.scrollIntoView({
					behavior: 'smooth'
				});
			}, 500);
		} catch (error) {
			errorMessage = error as string;
			showError = true;
			console.error('Lỗi:', error);
		} finally {
			isProcessing = false;
		}
	}

	function removeFile() {
		uploadedFile = null;
		fileInput.value = '';
	}

	async function handleExport() {
		try {
			// Đọc giá trị từ configs.json
			const configContent = await readTextFile('configs.json');
			const config = JSON.parse(configContent);
			const threshold = config.Value / (-2 / 35);

			console.log('Ngưỡng xuất file:', threshold.toFixed(2) + '%');

			isExporting = true;
			// Sửa lại logic lọc - giữ lại những câu có tỉ lệ trùng THẤP HƠN ngưỡng
			const duplicateQuestions = similarities.filter((item) => {
				const similarityPercentage = parseFloat(item.similarity_score.replace('%', ''));
				console.log(
					`Câu ${item.id}: ${similarityPercentage}% ${similarityPercentage >= threshold ? '(Loại bỏ)' : '(Giữ lại)'}`
				);
				return similarityPercentage < threshold; // Thay đổi từ >= thành <
			});

			const duplicateIds = duplicateQuestions.map((item) => item.id);

			try {
				const newFilePath = processedFilePath.replace('.docx', '_filtered.docx');
				await invoke('filter_docx', {
					filePath: processedFilePath,
					duplicateIds: duplicateIds
				});

				console.log('Đã tạo file mới tại:', newFilePath);
				console.log('Số câu được giữ lại:', duplicateIds.length);
				showSuccess = true;
				successType = 'export';
			} catch (error) {
				console.error('Lỗi khi xử lý file:', error);
				errorMessage = 'Có lỗi xảy ra khi xử lý file';
				showError = true;
			} finally {
				isExporting = false;
			}
		} catch (error) {
			console.error('Lỗi khi đọc file config:', error);
			errorMessage = 'Có lỗi xảy ra khi đọc cấu hình';
			showError = true;
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
		class="w-[1000px] animate-[zoomIn_0.6s_ease-in-out] rounded-lg border border-white/20 bg-white/90 p-16 shadow-2xl backdrop-blur-sm transition-all duration-300 hover:shadow-blue-500/10"
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
					class="w-full rounded-lg bg-gradient-to-r from-[#8E7FDD] to-[#CCABD6] py-3 font-medium text-white transition-all duration-300 hover:shadow-lg hover:shadow-[#8E7FDD]/30 disabled:cursor-not-allowed disabled:opacity-50"
					on:click={processFile}
					disabled={isProcessing}
				>
					{#if isProcessing}
						<div class="flex items-center justify-center gap-2">
							<div
								class="h-5 w-5 animate-spin rounded-full border-2 border-white border-t-transparent"
							></div>
							<span>Đang xử lý...</span>
						</div>
					{:else}
						Kiểm tra
					{/if}
				</button>
			</div>
		{/if}
	</div>
</div>

{#if showResults}
	<div id="results-section" class="container mx-auto min-h-screen max-w-[1200px] px-4 pb-20 pt-8">
		<div class="space-y-6" id="duplicate-section">
			{#each similarities as item}
				<div
					class="rounded-lg border bg-white/90 p-8 transition-all duration-300 hover:scale-[1.02] hover:shadow-lg"
				>
					<div class="flex-1">
						<div class="mb-6 border-b pb-4">
							<p class="mb-2 text-lg font-medium">
								Câu {item.id}: {item.docx_question}
							</p>
							<div class="mt-2 space-y-1">
								<p class="mb-2 font-medium text-gray-700">Các phương án:</p>
								<div class="grid gap-2">
									{#each item.answers?.filter((answer: string) => {
										const content = answer.split('. ')[1];
										return content && content.trim() !== '';
									}) || [] as answer}
										{@const answerKey = answer.split('.')[0].trim().toUpperCase()}
										<div
											class="flex items-center rounded-lg border p-3 transition-all duration-200 hover:shadow-sm {item.correct_answer_keys &&
											item.correct_answer_keys.includes(answerKey)
												? 'bg-[#8E7FDD] bg-opacity-10'
												: ''}"
											class:border-[#8E7FDD]={item.correct_answer_keys &&
												item.correct_answer_keys.includes(answerKey)}
											class:border-gray-200={!(
												item.correct_answer_keys && item.correct_answer_keys.includes(answerKey)
											)}
										>
											<div
												class="mr-3 flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full border-2"
												class:border-[#8E7FDD]={item.correct_answer_keys &&
													item.correct_answer_keys.includes(answerKey)}
												class:border-gray-300={!(
													item.correct_answer_keys && item.correct_answer_keys.includes(answerKey)
												)}
											>
												{#if item.correct_answer_keys && item.correct_answer_keys.includes(answerKey)}
													<div class="h-3 w-3 rounded-full bg-[#8E7FDD]"></div>
												{/if}
											</div>
											<p
												class="text-gray-700"
												class:font-medium={item.correct_answer_keys &&
													item.correct_answer_keys.includes(answerKey)}
											>
												{answer}
											</p>
										</div>
									{/each}
								</div>
							</div>
						</div>
						<div class="mt-4 flex items-center gap-2">
							<span class="text-base text-gray-600">Trạng thái:</span>
							{#if item.is_similar}
								{#if item.duplicate_type === 'docx'}
									<span class="rounded-full bg-yellow-500 px-3 py-1 text-sm font-medium text-white">
										Trùng trong file Docx
									</span>
									{#if !isNaN(item.question_similarity) && !isNaN(item.answer_similarity)}
										<span class="text-sm text-gray-600">
											Q: {(item.question_similarity * 100).toFixed(2)}% | A: {(
												item.answer_similarity * 100
											).toFixed(2)}%
										</span>
									{/if}
								{:else if item.duplicate_type === 'db'}
									<span class="rounded-full bg-red-500 px-3 py-1 text-sm font-medium text-white">
										Trùng trong Database
									</span>
									{#if !isNaN(item.question_similarity) && !isNaN(item.answer_similarity)}
										<span class="text-sm text-gray-600">
											Q: {(item.question_similarity * 100).toFixed(2)}% | A: {(
												item.answer_similarity * 100
											).toFixed(2)}%
										</span>
									{/if}
								{/if}
							{:else}
								<span class="rounded-full bg-green-500 px-3 py-1 text-sm font-medium text-white">
									Không trùng lặp
								</span>
							{/if}
						</div>
					</div>
				</div>
			{/each}

			{#if similarities.length === 0}
				<div class="py-8 text-center text-gray-600">
					<p class="text-lg">Không có câu hỏi nào</p>
				</div>
			{/if}
		</div>

		<!-- Nút xuất Docx -->
		<div class="mt-8 flex justify-center pb-8">
			<button
				class="flex items-center gap-2 rounded-lg bg-gradient-to-r from-[#8E7FDD] to-[#CCABD6] px-6 py-3 font-medium text-white transition-all duration-300 hover:shadow-lg hover:shadow-[#8E7FDD]/30"
				on:click={handleExport}
				disabled={isExporting}
			>
				{#if isExporting}
					<div
						class="h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent"
					></div>
					<span>Đang xuất file...</span>
				{:else}
					<Icon icon="icon-park-outline:file-pdf" class="h-5 w-5" />
					<span>Xuất Docx</span>
				{/if}
			</button>
		</div>
	</div>
{/if}

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
			<span class="text-sm text-white/80">
				{#if successType === 'check'}
					Đã hoàn thành kiểm tra trùng lặp câu hỏi
				{:else}
					Đã xuất file thành công!
				{/if}
			</span>
		</div>
		<button
			class="ml-4 rounded-full bg-white/20 p-1 hover:bg-white/30"
			on:click={() => (showSuccess = false)}
		>
			<Icon icon="icon-park-outline:close" class="h-4 w-4" />
		</button>
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
		<button
			class="ml-4 rounded-full bg-white/20 p-1 hover:bg-white/30"
			on:click={() => (showError = false)}
		>
			<Icon icon="icon-park-outline:close" class="h-4 w-4" />
		</button>
	</div>
{/if}

<input
	type="file"
	accept=".docx"
	class="hidden"
	bind:this={fileInput}
	on:change={handleFileSelect}
/>

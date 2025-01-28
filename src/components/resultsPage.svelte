<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	let activeTab = 'duplicate';

	function switchTab(tab: string) {
		activeTab = tab;
	}
</script>

<div class="mx-auto mt-8 w-full max-w-[2000px]">
	<div class="rounded-lg bg-white shadow-md">
		<div class="flex w-full border-b">
			<button
				class="flex-1 px-8 py-4 text-base font-medium {activeTab === 'duplicate'
					? 'border-b-2 border-[#8E7FDD] text-[#8E7FDD]'
					: 'text-gray-500 hover:text-gray-700'}"
				on:click={() => switchTab('duplicate')}
			>
				Duplicate
			</button>
			<button
				class="flex-1 px-8 py-4 text-base font-medium {activeTab === 'approve'
					? 'border-b-2 border-[#8E7FDD] text-[#8E7FDD]'
					: 'text-gray-500 hover:text-gray-700'}"
				on:click={() => switchTab('approve')}
			>
				Approved
			</button>
		</div>

		<div class="p-10">
			{#if activeTab === 'duplicate'}
				<slot name="duplicate" />
			{:else}
				<slot name="approve" />
			{/if}
		</div>
	</div>
</div>

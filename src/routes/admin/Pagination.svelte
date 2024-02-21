<script lang="ts">
	import * as Pagination from "$lib/shadcn/ui/pagination";
	export let total: number;
	export let pageNumber: number;
	export let getUsers: () => Promise<void>;
</script>

<Pagination.Root count={total} perPage={10} let:pages let:currentPage page={pageNumber + 1}>
	<Pagination.Content>
		<Pagination.Item>
			<Pagination.PrevButton on:click={() => {
				pageNumber--;
				getUsers();
			}}/>
		</Pagination.Item>
		{#each pages as page (page.key)}
			{#if page.type === "ellipsis"}
				<Pagination.Item>
					<Pagination.Ellipsis />
				</Pagination.Item>
			{:else}
				<Pagination.Item isVisible={currentPage === page.value}>
					<Pagination.Link {page} isActive={currentPage === page.value} on:click={() => {
						pageNumber = page.value - 1;
						getUsers();
					}}>
						{page.value}
					</Pagination.Link>
				</Pagination.Item>
			{/if}
		{/each}
		<Pagination.Item>
			<Pagination.NextButton on:click={() => {
				pageNumber++;
				getUsers();
			}}/>
		</Pagination.Item>
	</Pagination.Content>
</Pagination.Root>
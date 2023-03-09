<script lang="ts">
	import NavigationButton from '$lib/components/NavigationButton.svelte';
	import TargetPlacement from '$lib/components/TargetPlacement.svelte';

	import { page } from '$app/stores';
</script>

<TargetPlacement>
	<NavigationButton back to="/visualise/{$page.params.slug}">
		Apply and go to {$page.params.slug}
	</NavigationButton>
</TargetPlacement>
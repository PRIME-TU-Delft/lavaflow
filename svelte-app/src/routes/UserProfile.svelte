<script lang="ts">
	export let name: string;
	export let image: string = 'default.jpeg';
	export let title: string;
</script>

<div class="flex gap-4">
	<img
		class="h-16 w-16 rounded-full object-cover"
		src="/profile-images/{image}"
		alt="profile for {name}"
	/>
	<div>
		<h4>{name}</h4>
		<p>
			{title}
		</p>
	</div>
</div>

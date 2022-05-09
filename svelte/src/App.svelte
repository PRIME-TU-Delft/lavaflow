<script>
  import NavigationButton from './NavigationButton.svelte';
  import { Router, Route } from 'svelte-navigator';
  import MapScanning from './MapScanning.svelte';
</script>

<Router primary={false}>
  <Route path="/">
    <header>
      <div class="img" />
      <div class="img backdrop" />
      <div class="title">
        <h1>PRIME</h1>
        <h2>AR Lavaflow</h2>
      </div>
    </header>

    <main>
      When a volcano erupts, the lava produced is unstoppable. It moves so
      slowly you can outrun it, but with the density of molten rock, and a
      temperature of over 1000 degrees Celsius, it will destroy everything in
      its path. This path being the steepest possible down a mountain. Your goal
      is to draw a mountain on a piece of paper. Scan it and place targets on
      the map where you predict the lava will reach.

      <NavigationButton large to="map-scanning">Start flowing</NavigationButton>
      <NavigationButton large to="/" secondary={true}>
        Teacher corner
      </NavigationButton>
    </main>
  </Route>

  <Route path="map-scanning"><MapScanning /></Route>
  <Route path="map-scanned"><h1>'Ello</h1></Route>
  <Route path="house-scanning" />
  <Route path="house-scanned" />
  <Route path="visualize-ar" />
  <Route path="visualize-vr" />
</Router>

<style>
  h1 {
    margin: 0;
    font-size: 2em;
  }

  h2 {
    font-size: 1.5em;
  }
  header {
    position: relative;
    height: 20rem;
    width: 100%;
  }

  main {
    display: block;
    max-width: 30rem;
    margin: 1rem auto;
    padding: 1rem;
  }

  .img {
    position: absolute;
    bottom: 0;
    background: url(/volcano.jpg);
    background-size: cover;
    background-position: center bottom;
    left: 50%;
    transform: translateX(-50%);
    height: 40rem;
    width: 40rem;
    z-index: 0;
    border-radius: 50%;
  }

  .backdrop {
    background: rgba(220, 73, 73, 21%);
    backdrop-filter: blur(1px) saturate(120%);
  }

  .title {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
    text-align: center;
    font-size: 1.5em;
  }
</style>

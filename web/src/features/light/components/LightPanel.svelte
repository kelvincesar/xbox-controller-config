<script lang="ts">
  import BrightnessSlider from './BrightnessSlider.svelte';
  import LightActions from './LightActions.svelte';
  import ModePicker from './ModePicker.svelte';
</script>

<fieldset>
  <legend>Light</legend>
  <ModePicker />
  <BrightnessSlider />
</fieldset>
<LightActions />

<style>
  fieldset {
    margin: 0;
    padding: 0;
    border: 0;
    min-width: 0;
    display: grid;
    gap: 1.5rem;
  }
</style>

import { quartOut } from 'svelte/easing';
import { fly, type TransitionConfig } from 'svelte/transition';

function reducedMotion(): boolean {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Entrance for content that replaces other content (a panel swap, an error):
 * a short rise and fade. Use as `in:rise` only; leaving content disappears at
 * once so the old and new never overlap.
 */
export function rise(node: Element): TransitionConfig {
  return fly(node, {
    y: 6,
    duration: reducedMotion() ? 0 : 220,
    easing: quartOut,
  });
}

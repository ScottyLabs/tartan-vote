<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    variant?: 'primary' | 'ghost' | 'dark';
    type?: 'button' | 'submit' | 'reset';
    disabled?: boolean;
    full?: boolean;
    size?: 'sm' | 'md' | 'lg';
    onclick?: (e: MouseEvent) => void;
    children: Snippet;
    class?: string;
  }
  let {
    variant = 'primary',
    type = 'button',
    disabled = false,
    full = false,
    size = 'md',
    onclick,
    children,
    class: klass = ''
  }: Props = $props();

  const base = {
    primary: 'btn-primary',
    ghost: 'btn-ghost',
    dark: 'btn-dark'
  }[variant];

  const sizeClass = {
    sm: 'text-[13px] px-3 py-2',
    md: 'text-sm px-5 py-3',
    lg: 'text-base px-6 py-3.5'
  }[size];
</script>

<button
  {type}
  {disabled}
  {onclick}
  class="{base} {sizeClass} {full ? 'w-full' : ''} {klass}"
>
  {@render children()}
</button>

<script lang="ts">
  interface Props {
    value: string;
    type?: string;
    placeholder?: string;
    required?: boolean;
    label?: string;
    id?: string;
    size?: 'sm' | 'md' | 'lg';
    autocomplete?: string;
    oninput?: (e: Event) => void;
    onkeydown?: (e: KeyboardEvent) => void;
    class?: string;
  }
  let {
    value = $bindable(),
    type = 'text',
    placeholder = '',
    required = false,
    label,
    id,
    size = 'md',
    autocomplete,
    oninput,
    onkeydown,
    class: klass = ''
  }: Props = $props();

  const sizeClass = size === 'lg' ? 'input input-lg' : 'input';
</script>

{#if label}
  <label for={id} class="label block mb-2">{label}</label>
{/if}
<input
  {id}
  {type}
  {placeholder}
  {required}
  {autocomplete}
  bind:value
  class="{sizeClass} {klass}"
  {oninput}
  {onkeydown}
/>

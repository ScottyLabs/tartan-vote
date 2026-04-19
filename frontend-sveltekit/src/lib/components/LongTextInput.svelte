<script lang="ts">
  interface Props {
    value: string;
    label?: string;
    placeholder?: string;
    required?: boolean;
    id?: string;
  }
  let {
    value = $bindable(''),
    label,
    placeholder = '',
    required = false,
    id
  }: Props = $props();

  function autoresize(e: Event) {
    const t = e.target as HTMLTextAreaElement;
    t.style.height = 'auto';
    t.style.height = t.scrollHeight + 'px';
  }
</script>

{#if label}<label for={id} class="label block mb-2">{label}</label>{/if}
<textarea
  {id}
  {placeholder}
  {required}
  bind:value
  oninput={autoresize}
  class="input min-h-[90px] resize-none"
></textarea>

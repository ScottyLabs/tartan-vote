<script lang="ts">
  interface Option {
    label: string;
    value: string | number;
  }
  interface Props {
    value: string | number;
    options: Array<string | number | Option>;
    label?: string;
    id?: string;
  }
  let { value = $bindable(), options, label, id }: Props = $props();

  const normalized = $derived(
    options.map((o) =>
      typeof o === 'object' && o !== null && 'label' in o
        ? (o as Option)
        : { label: String(o), value: o as string | number }
    )
  );
</script>

{#if label}<label for={id} class="label block mb-2">{label}</label>{/if}
<select {id} bind:value class="input cursor-pointer">
  {#each normalized as opt}
    <option value={opt.value}>{opt.label}</option>
  {/each}
</select>

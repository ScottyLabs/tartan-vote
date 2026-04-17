<script lang="ts">
    type PrimitiveOption = string | number;
    type ObjectOption = { label: string; value: string | number };
    type SelectOption = PrimitiveOption | ObjectOption;

    let {
        title = "Popup",
        value = $bindable(),
        options = [] as SelectOption[],
    } = $props();

    const normalizedOptions = $derived(
        options.map((option) =>
            typeof option === "object" && option !== null
                ? option
                : { label: String(option), value: option },
        ),
    );
</script>

<main>
    <h3>{title}</h3>
    <select bind:value required>
        <option value="" disabled>Select one...</option>
        {#each normalizedOptions as option}
            <option value={option.value}>{option.label}</option>
        {/each}
    </select>
</main>

<style>
    h3 {
        text-align: left;
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }
    select {
        width: 100%;
        height: 50px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid var(--color-border);
        box-sizing: border-box;
        font-size: 20px;
        margin-bottom: 0em;
    }
</style>

<script>
    let { sessionCode } = $props();
</script>

<main>
    <h1>Waiting for Host to Start a Motion...</h1>
    <h2>Session Code: {sessionCode}</h2>
    <div
        class="spinner"
        style="width: 40px; height: 40px; border-top-color: #FF3B3F;"
    ></div>
</main>

<style>
    .spinner {
        border: 3px solid #ddd;
        border-radius: 50%;
        border-top-color: #555;
        animation: spin 1s linear infinite;
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 1000;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>

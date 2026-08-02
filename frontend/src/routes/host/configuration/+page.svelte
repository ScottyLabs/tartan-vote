<script lang="ts">
  import { page } from "$app/state";
  import Link2 from "@lucide/svelte/icons/link-2";
  import Pencil from "@lucide/svelte/icons/pencil";
  import { onMount } from "svelte";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import ConfirmDeleteOrganizationDialog from "$lib/components/ConfirmDeleteOrganizationDialog.svelte";
  import EndSessionDialog from "$lib/components/EndSessionDialog.svelte";
  import OrganizationNameDialog from "$lib/components/OrganizationNameDialog.svelte";
  import {
    clearOrganizationSettings,
    defaultOrganizationSettings,
    exampleOrganizations,
    loadDeletedOrganizationNames,
    loadOrganizationSettings,
    saveDeletedOrganizationNames,
    saveOrganizationSettings,
    type OrganizationSettings,
  } from "$lib/domain/organizationSettings";

  let organization = $state("Default");
  let defaultQuorum = $state("");
  let quickVoteOptions = $state("Yes, No");
  let approvedMembers = $state("");
  let saved = $state(false);
  let hasUnsavedChanges = $state(false);
  let dropdownOpen = $state(page.url.searchParams.get("dropdown") === "true");
  let error = $state("");
  let organizations = $state<OrganizationSettings[]>(
    exampleOrganizations.map((settings) => ({
      name: settings.name,
      quorum: settings.quorum,
      quickVoteOptions: settings.quickVoteOptions,
      approvedMembers: settings.approvedMembers,
    })),
  );
  let organizationPendingDelete = $state<string | null>(
    page.url.searchParams.get("delete"),
  );
  let nameDialogMode = $state<"create" | "edit" | null>(null);
  let nameDialogError = $state("");
  let endSessionDialogOpen = $state(false);

  const isDefault = $derived(organization === "Default");

  onMount(() => {
    const deletedNames = loadDeletedOrganizationNames();
    organizations = organizations.filter(
      (settings) => !deletedNames.includes(settings.name),
    );
    const storedSettings = loadOrganizationSettings();
    if (
      storedSettings.name !== "Default" &&
      !organizations.some((settings) => settings.name === storedSettings.name)
    ) {
      organizations.push(storedSettings);
    }
    const requestedOrganization = page.url.searchParams.get("organization");
    if (requestedOrganization) {
      selectOrganization(requestedOrganization);
      dropdownOpen = page.url.searchParams.get("dropdown") === "true";
      return;
    }
    applyOrganization(storedSettings);
  });

  function applyOrganization(settings: OrganizationSettings) {
    organization = settings.name;
    defaultQuorum = normalizeQuorum(settings.quorum);
    quickVoteOptions = settings.quickVoteOptions.join(", ");
    approvedMembers = settings.approvedMembers.join(", ");
    dropdownOpen = false;
    hasUnsavedChanges = false;
    saved = false;
    error = "";
  }

  function selectOrganization(name: string) {
    const settings =
      organizations.find((candidate) => candidate.name === name) ??
      defaultOrganizationSettings;
    applyOrganization(settings);
  }

  function addOrganization() {
    dropdownOpen = false;
    nameDialogError = "";
    nameDialogMode = "create";
  }

  function openEditOrganizationDialog() {
    if (isDefault) return;
    nameDialogError = "";
    nameDialogMode = "edit";
  }

  function submitOrganizationName(name: string) {
    if (!name) {
      nameDialogError = "Enter an organization name.";
      return;
    }
    const duplicate = organizations.some(
      (settings) =>
        settings.name.toLocaleLowerCase() === name.toLocaleLowerCase() &&
        (nameDialogMode === "create" || settings.name !== organization),
    );
    if (duplicate) {
      nameDialogError = "An organization with this name already exists.";
      return;
    }

    if (nameDialogMode === "create") {
      applyOrganization({
        name,
        quorum: "",
        quickVoteOptions: ["Yes", "No"],
        approvedMembers: [],
      });
    } else {
      const oldName = organization;
      organization = name;
      organizations = organizations.map((settings) =>
        settings.name === oldName
          ? {
              name,
              quorum: settings.quorum,
              quickVoteOptions: settings.quickVoteOptions,
              approvedMembers: settings.approvedMembers,
            }
          : settings,
      );
      saved = false;
    }
    hasUnsavedChanges = true;
    nameDialogMode = null;
    nameDialogError = "";
  }

  function requestOrganizationDeletion(name: string) {
    organizationPendingDelete = name;
    dropdownOpen = false;
  }

  function deleteOrganization() {
    if (!organizationPendingDelete || organizationPendingDelete === "Default")
      return;
    const deletedName = organizationPendingDelete;
    organizations = organizations.filter(
      (settings) => settings.name !== deletedName,
    );
    saveDeletedOrganizationNames([
      ...new Set([...loadDeletedOrganizationNames(), deletedName]),
    ]);
    if (organization === deletedName) {
      clearOrganizationSettings();
      applyOrganization(defaultOrganizationSettings);
    }
    organizationPendingDelete = null;
  }

  function normalizeQuorum(value: string) {
    const digits = value.replaceAll(/\D/gu, "").slice(0, 5);

    return digits !== "" && Number(digits) > 0 ? digits : "";
  }

  function parseCommaList(value: string): string[] {
    return value
      .split(",")
      .map((item) => item.trim())
      .filter((item) => item !== "");
  }

  function hasUniqueIgnoreCase(items: string[]): boolean {
    return new Set(items.map((item) => item.toLocaleLowerCase())).size === items.length;
  }

  function keepDigits(event: Event) {
    defaultQuorum = normalizeQuorum(
      (event.currentTarget as HTMLInputElement).value,
    );
    saved = false;
    hasUnsavedChanges = true;
  }

  function keepQuickVoteOptions(event: Event) {
    quickVoteOptions = (event.currentTarget as HTMLInputElement).value
      .split(",")
      .slice(0, 10)
      .map((option) => option.slice(0, 36))
      .join(",");
    saved = false;
    hasUnsavedChanges = true;
  }

  function keepApprovedMembers(event: Event) {
    approvedMembers = (event.currentTarget as HTMLTextAreaElement).value
      .split(",")
      .slice(0, 200)
      .map((andrewId) => andrewId.slice(0, 36))
      .join(",");
    saved = false;
    hasUnsavedChanges = true;
  }

  function saveSettings(event: SubmitEvent) {
    event.preventDefault();
    if (isDefault) return;
    if (organization.trim() === "") {
      error = "Enter an organization name.";
      return;
    }
    const options = parseCommaList(quickVoteOptions);
    if (options.length < 2) {
      error = "Enter at least two Quick Vote options.";
      return;
    }
    if (!hasUniqueIgnoreCase(options)) {
      error = "Each Quick Vote option must be unique.";
      return;
    }
    const members = parseCommaList(approvedMembers);
    if (!hasUniqueIgnoreCase(members)) {
      error = "Each approved AndrewID must be unique.";
      return;
    }
    const savedSettings = {
      name: organization.trim(),
      quorum: defaultQuorum,
      quickVoteOptions: options,
      approvedMembers: members,
    };
    saveOrganizationSettings(savedSettings);
    saveDeletedOrganizationNames(
      loadDeletedOrganizationNames().filter((name) => name !== savedSettings.name),
    );
    organizations = [
      ...organizations.filter((settings) => settings.name !== savedSettings.name),
      savedSettings,
    ];
    error = "";
    saved = true;
    hasUnsavedChanges = false;
  }
</script>

<svelte:head>
  <title>TartanVote | Session Configuration</title>
</svelte:head>

  <div
    class="grid min-h-[calc(100svh-clamp(60px,5.833vw,112px)-81px)] flex-[1_0_calc(100svh-clamp(60px,5.833vw,112px)-81px)] grid-cols-[clamp(203px,19.792vw,380px)_minmax(0,1fr)]"
  >
    <aside
      class="relative min-h-[calc(100svh-clamp(60px,5.833vw,112px)-81px)] bg-grey-900 text-white"
      aria-label="Host dashboard navigation"
    >
      <section
        class="h-[clamp(115px,11.198vw,215px)] bg-grey-800 px-[clamp(14px,1.354vw,26px)] pt-[clamp(22px,2.188vw,42px)]"
        aria-labelledby="session-code-label"
      >
        <h2
          class="mt-0 mb-[clamp(4px,1.042vw,20px)] text-[clamp(10px,0.938vw,18px)] leading-[clamp(14px,1.354vw,26px)] text-grey-500"
          id="session-code-label"
        >
          Session Code
        </h2>
        <p
          class="mt-0 mb-[clamp(6px,1.667vw,32px)] text-[clamp(17px,1.563vw,30px)] leading-[clamp(22px,1.979vw,38px)] font-semibold"
        >
          happy-giraffe
        </p>
        <button
          class="inline-flex h-6 max-w-full cursor-pointer items-center gap-2 border-0 bg-transparent p-0 text-[clamp(9px,0.833vw,16px)] text-red-200"
          type="button"
        >
          <span class="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap"
            >Copy invite link</span
          >
          <span
            class="grid size-[clamp(10px,1.042vw,20px)] shrink-0 place-items-center"
            aria-hidden="true"
            ><Link2
              class="block size-full text-red-200"
              aria-hidden="true"
            /></span
          >
        </button>
      </section>

      <nav
        class="mt-[clamp(12px,1.198vw,23px)] grid gap-[5px]"
        aria-label="Session sections"
      >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href="/host">Overview</a
        >
        <button
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-not-allowed items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-grey-100 opacity-100"
          type="button"
          disabled
          title="Live View is available while a vote is running"
          >Live View</button
        >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center bg-[#66363e] px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href="/host/configuration"
          aria-current="page">Session Configuration</a
        >
        <button
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white"
          type="button">Comprehensive Session Results</button
        >
      </nav>

      <button
        class="absolute bottom-[35px] left-1/2 h-[clamp(34px,3.125vw,60px)] w-[min(calc(100%-42px),300px)] -translate-x-1/2 cursor-pointer rounded-[100px] border border-red-500 bg-transparent text-[clamp(14px,1.25vw,24px)] text-grey-200"
        type="button"
        onclick={() => (endSessionDialogOpen = true)}>End session</button
      >
    </aside>

    <section
      class="flex flex-col items-center justify-center-safe px-[clamp(19px,1.823vw,35px)] pt-[clamp(16px,1.719vw,33px)] pb-[clamp(10px,0.938vw,18px)]"
      aria-labelledby="configuration-title"
    >
      <h1
        class="mt-0 mb-[clamp(6px,1.042vw,20px)] w-full text-center text-[clamp(25px,2.344vw,45px)] leading-[clamp(32px,2.917vw,56px)] font-semibold"
        id="configuration-title"
      >
        Session Configuration
      </h1>

      <form
        class="w-full max-w-[1470px] rounded-[10px] border-2 border-grey-200 bg-white px-[clamp(20px,2.083vw,40px)] pt-[clamp(12px,1.979vw,38px)] pb-[clamp(22px,2.083vw,40px)]"
        onsubmit={saveSettings}
      >
        <header class="flex items-start justify-between">
          <div>
            <h2
              class="mt-0 mb-[3px] text-[clamp(14px,1.25vw,24px)] leading-[clamp(20px,1.667vw,32px)] font-medium text-black"
            >
              Customize Room (Optional)
            </h2>
            <p
              class="m-0 text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-grey-700"
            >
              Configure organization specific settings
            </p>
          </div>
          <button
            class="inline-flex cursor-pointer items-center gap-1.5 border-0 bg-transparent p-0 text-[clamp(9px,0.938vw,18px)] text-grey-700 underline disabled:cursor-not-allowed disabled:opacity-72"
            type="button"
            disabled={isDefault}
            onclick={openEditOrganizationDialog}
          >
            <Pencil
              class="size-[clamp(12px,1.094vw,21px)] text-grey-700"
              strokeWidth={2.5}
              aria-hidden="true"
            />
            <span
              >{isDefault
                ? "Default cannot be edited"
                : "Edit Organization Name"}</span
            >
          </button>
        </header>

        {#if isDefault}
          <p
            class="mt-2 mb-[-4px] text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] italic text-grey-600"
          >
            Default settings are read-only. Add another organization to create
            and save custom settings.
          </p>
        {/if}

        <div
          class="mt-[clamp(14px,1.458vw,28px)] grid grid-cols-[clamp(210px,20.833vw,400px)_minmax(0,1fr)] gap-[clamp(30px,3.646vw,70px)]"
        >
          <div class="grid gap-[clamp(12px,1.042vw,20px)]">
            <label>
              <span
                class="block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-black"
                >Selected Organization:</span
              >
              <small
                class="m-0 block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-grey-700"
                >Configure an organization or use Default settings</small
              >
              <div class="relative mt-[5px] w-[clamp(160px,15.781vw,303px)]">
                <button
                  class="flex h-[clamp(28px,2.604vw,50px)] w-full cursor-pointer items-center justify-between rounded-[10px] border border-black bg-white px-3.5 text-[clamp(9px,0.833vw,16px)] font-medium text-black"
                  type="button"
                  aria-haspopup="listbox"
                  aria-expanded={dropdownOpen}
                  onclick={() => (dropdownOpen = !dropdownOpen)}
                >
                  <span>{organization}</span><span aria-hidden="true">></span>
                </button>

                {#if dropdownOpen}
                  <div
                    class="absolute top-[calc(100%+1px)] left-0 z-5 w-full overflow-hidden rounded-[5px] border-[0.5px] border-grey-100 bg-white py-0.5 shadow-[0_2px_3px_rgb(0_0_0_/_0.25)]"
                    role="listbox"
                    aria-label="Organizations"
                  >
                    {#each organizations as settings}
                      <div
                        class:bg-grey-100={settings.name === organization}
                        class="grid h-[clamp(28px,2.604vw,50px)] grid-cols-[38px_minmax(0,1fr)] items-center bg-white hover:bg-grey-100"
                      >
                        {#if settings.name !== "Default"}
                          <button
                            class="flex h-[clamp(28px,2.604vw,50px)] w-[38px] cursor-pointer items-center justify-center border-0 bg-transparent p-0 text-[clamp(16px,1.25vw,24px)] leading-none font-medium text-grey-500 hover:text-red-500"
                            type="button"
                            aria-label={`Delete ${settings.name}`}
                            onclick={() =>
                              requestOrganizationDeletion(settings.name)}
                          >
                            x
                          </button>
                        {:else}
                          <span class="w-[38px]" aria-hidden="true"></span>
                        {/if}
                        <button
                          class="flex h-[clamp(28px,2.604vw,50px)] w-full cursor-pointer items-center border-0 bg-transparent pr-3.5 text-left text-[clamp(9px,0.833vw,16px)] font-medium text-black"
                          type="button"
                          role="option"
                          aria-selected={settings.name === organization}
                          onclick={() => selectOrganization(settings.name)}
                        >
                          {settings.name}
                        </button>
                      </div>
                    {/each}
                    <button
                      class="flex h-[clamp(28px,2.604vw,50px)] w-full cursor-pointer items-center border-0 bg-transparent px-3.5 text-left text-[clamp(9px,0.833vw,16px)] font-medium text-grey-400 italic"
                      type="button"
                      onclick={addOrganization}
                      >+ Add another organization</button
                    >
                  </div>
                {/if}
              </div>
            </label>

            <label>
              <span
                class="block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-black"
                >Default Organization Quorum:</span
              >
              <small
                class="m-0 block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-grey-700"
                >Input the default minimum number of active participants
                required to begin a vote.</small
              >
              <input
                class="mt-[5px] h-[clamp(28px,2.604vw,50px)] w-[clamp(160px,15.781vw,303px)] rounded-[10px] border border-black bg-white px-3.5 text-[clamp(9px,0.833vw,16px)] text-black placeholder:text-grey-400 placeholder:opacity-100 disabled:cursor-not-allowed disabled:bg-grey-50 disabled:text-grey-400"
                value={defaultQuorum}
                inputmode="numeric"
                pattern="[0-9]*"
                maxlength="5"
                placeholder="N/A"
                aria-label="Default organization quorum"
                oninput={keepDigits}
                disabled={isDefault}
              />
            </label>

            <label>
              <span
                class="block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-black"
                >Quick Vote Voting Options:</span
              >
              <small
                class="m-0 block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-grey-700"
                >Input the default quick voting selections separated by a comma.</small
              >
              <input
                class="mt-[5px] h-[clamp(28px,2.604vw,50px)] w-[clamp(160px,15.781vw,303px)] rounded-[10px] border border-black bg-white px-3.5 text-[clamp(9px,0.833vw,16px)] text-black disabled:cursor-not-allowed disabled:bg-grey-50 disabled:text-grey-400"
                value={quickVoteOptions}
                maxlength="369"
                oninput={keepQuickVoteOptions}
                disabled={isDefault}
              />
            </label>
          </div>

          <label>
            <span
              class="block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-black"
              >Approved Voting Members:</span
            >
            <small
              class="m-0 block text-[clamp(9px,0.833vw,16px)] leading-[clamp(13px,1.25vw,24px)] font-medium text-grey-700"
              >Input the AndrewID(s) of allowed participants separated by a
              comma. Leave blank to allow unrestricted session joining.</small
            >
            <textarea
              class="mt-2.5 h-[clamp(182px,17.813vw,342px)] min-h-[clamp(182px,17.813vw,342px)] max-h-[520px] w-full resize-y rounded-[10px] border border-black bg-white px-4 py-3.5 text-[clamp(9px,0.833vw,16px)] text-black placeholder:text-grey-400 placeholder:opacity-100 disabled:cursor-not-allowed disabled:bg-grey-50 disabled:text-grey-400"
              value={approvedMembers}
              maxlength="7399"
              placeholder="N/A"
              oninput={keepApprovedMembers}
              disabled={isDefault}></textarea>
          </label>
        </div>

        <div
          class="mt-[clamp(16px,1.563vw,30px)] grid min-h-6 place-items-center"
          aria-live="polite"
        >
          {#if error}<p
              class="m-0 text-center text-[clamp(11px,0.729vw,14px)] text-red-700"
              role="alert"
            >
              {error}
            </p>{/if}
          {#if hasUnsavedChanges}
            <p
              class="m-0 text-center text-sm font-semibold text-red-500"
              role="status"
            >
              You have unsaved changes
            </p>
          {/if}
          {#if saved}<p
              class="m-0 text-center text-sm font-semibold text-green-600"
            >
              Settings saved.
            </p>{/if}
        </div>
        <button
          class="mx-auto block h-[clamp(34px,3.125vw,60px)] w-[clamp(268px,26.042vw,500px)] cursor-pointer rounded-[100px] border-0 bg-red-600 text-[clamp(14px,1.25vw,24px)] font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-grey-400 disabled:shadow-none"
          type="submit"
          disabled={isDefault}>Save Settings</button
        >
      </form>
    </section>
  </div>

  <AppFooter wide flow />

{#if organizationPendingDelete}
  <ConfirmDeleteOrganizationDialog
    organizationName={organizationPendingDelete}
    onclose={() => (organizationPendingDelete = null)}
    ondelete={deleteOrganization}
  />
{/if}

{#if nameDialogMode}
  <OrganizationNameDialog
    mode={nameDialogMode}
    initialName={nameDialogMode === "edit" ? organization : ""}
    error={nameDialogError}
    onclose={() => (nameDialogMode = null)}
    onsubmit={submitOrganizationName}
  />
{/if}

{#if endSessionDialogOpen}
  <EndSessionDialog
    onclose={() => (endSessionDialogOpen = false)}
    onconfirm={() => (window.location.href = "/home")}
  />
{/if}

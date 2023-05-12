<script lang="ts">
import { defineAsyncComponent, onMounted, ref, watch } from 'vue';
import { StorageResponse, TeleportResponse } from './types';
import { useStorage, useTeleport } from './server';
import { removeQuotes } from './helpers';
import ConnectionPanel from './components/ConnectionPanel.vue';
const Button = defineAsyncComponent(() => import('./components/Button.vue'));
const ButtonGroup = defineAsyncComponent(
  () => import('./components/ButtonGroup.vue'),
);
const Checkbox = defineAsyncComponent(
  () => import('./components/Checkbox.vue'),
);
const Descriptive = defineAsyncComponent(
  () => import('./components/Descriptive.vue'),
);
const DirectoryChooser = defineAsyncComponent(
  () => import('./components/DirectoryChooser.vue'),
);
const Flex = defineAsyncComponent(() => import('./components/Flex.vue'));
const FunctionalPanel = defineAsyncComponent(
  () => import('./components/FunctionalPanel.vue'),
);
const Grid = defineAsyncComponent(() => import('./components/Grid.vue'));
const GridItem = defineAsyncComponent(
  () => import('./components/GridItem.vue'),
);
const Idol = defineAsyncComponent(() => import('./components/Idol.vue'));
const FolderTransferIcon = defineAsyncComponent(
  () => import('./components/Icon/FolderTransferIcon.vue'),
);
const FolderDestinationIcon = defineAsyncComponent(
  () => import('./components/Icon/FolderDestinationIcon.vue'),
);
const List = defineAsyncComponent(() => import('./components/List.vue'));
const ListItem = defineAsyncComponent(
  () => import('./components/ListItem.vue'),
);
const Modal = defineAsyncComponent(() => import('./components/Modal.vue'));
const ModalFooter = defineAsyncComponent(
  () => import('./components/ModalFooter.vue'),
);
const StoragePanel = defineAsyncComponent(
  () => import('./components/StoragePanel.vue'),
);
const TeleportPanel = defineAsyncComponent(
  () => import('./components/TeleportPanel.vue'),
);
const TitleHeader = defineAsyncComponent(
  () => import('./components/TitleHeader.vue'),
);
const TrashIcon = defineAsyncComponent(
  () => import('./components/Icon/TrashIcon.vue'),
);
</script>

<script setup lang="ts">
const open = ref<boolean>(false);
const teleport = ref<string | string[]>('');
const storage = ref<string | string[]>('');
const teleports = ref<TeleportResponse[] | undefined>([]);
const storages = ref<StorageResponse[] | undefined>([]);
const { getTeleports, createTeleport } = useTeleport();
const { getStorages, createStorage } = useStorage();

function retrieveTeleports() {
  getTeleports()
    .then((res) => (teleports.value = res))
    .catch((e) => console.log(e));
}

function retrieveStorages() {
  getStorages()
    .then((res) => (storages.value = res))
    .catch((e) => console.log(e));
}

function createNewTeleport() {
  // Get the last element which is the folder name
  const name = `${teleport.value}`.split('/').at(-1) as string;
  let directories: string[] = [];

  if (Array.isArray(teleport.value)) directories = teleport.value;
  else directories.push(teleport.value);

  createTeleport({ name, directories })
    .then((rs) => console.log(rs))
    .catch((e) => console.log(e));
}

function createNewStorage() {
  const name = `${storage.value}`.split('/').at(-1) as string;
  let directories: string[] = [];

  if (Array.isArray(storage.value)) directories = storage.value;
  else directories.push(storage.value);

  directories.forEach((directory) =>
    createStorage({ name, directory })
      .then((rs) => console.log(rs))
      .catch((e) => console.log(e)),
  );
}

watch(teleport, (newTeleport, _oldTeleport) => {
  if (newTeleport.length === 0) return;
  createNewTeleport();
  retrieveTeleports();
});

watch(storage, (newStorage, _oldStorage) => {
  if (newStorage.length === 0) return;
  createNewStorage();
  retrieveStorages();
});

onMounted(() => {
  retrieveTeleports();
  retrieveStorages();
});
</script>

<template>
  <Grid type="default">
    <GridItem position="lheader">
      <Flex
        justify-content="center"
        align-items="center"
      >
        <TitleHeader
          title="Teleported Folders"
          text-position="center"
        />
      </Flex>
    </GridItem>
    <GridItem position="rheader">
      <Flex
        justify-content="center"
        align-items="center"
      >
        <TitleHeader
          title="Storage Folders"
          text-position="center"
        />
      </Flex>
    </GridItem>
    <GridItem position="main">
      <Flex justify-content="flex-end">
        <TeleportPanel>
          <List>
            <ListItem
              v-for="(teleport, index) in teleports"
              :id="teleport.index"
              :key="teleport.index"
            >
              <Idol>
                <FolderTransferIcon />
              </Idol>
              <Descriptive
                :title="removeQuotes(teleport.name)"
                :description="teleport.directories"
              />
              <Button
                rounded
                :name="'teleport-trash-btn-' + index"
                color="danger"
              >
                <TrashIcon />
              </Button>
            </ListItem>
          </List>
        </TeleportPanel>
      </Flex>
    </GridItem>
    <GridItem position="aside">
      <Flex justify-content="flex-start">
        <StoragePanel>
          <List>
            <ListItem
              v-for="(storage, index) in storages"
              :id="storage.index"
              :key="storage.index"
            >
              <Idol>
                <FolderDestinationIcon />
              </Idol>
              <Descriptive
                :title="removeQuotes(storage.name)"
                :description="removeQuotes(storage.directory)"
              />
              <Button
                rounded
                :name="'storage-trash-btn-' + index"
                color="danger"
              >
                <TrashIcon />
              </Button>
            </ListItem>
          </List>
        </StoragePanel>
      </Flex>
    </GridItem>
    <GridItem position="footer">
      <Flex justify-content="center">
        <FunctionalPanel>
          <Checkbox label="Start along side with OS" />
          <Checkbox label="Auto-scan" />
          <DirectoryChooser
            label="New Teleport"
            name="teleport"
            v-model:select="teleport"
          />
          <DirectoryChooser
            label="New Storage"
            name="storage"
            v-model:select="storage"
          />
          <ButtonGroup>
            <Button
              name="scan-btn"
              label="Scan Teleport"
              color="darker"
              larger
            />
            <Button
              name="choose-btn"
              label="Choose"
              color="darker"
              larger
              @click="open = true"
            />
          </ButtonGroup>
        </FunctionalPanel>
      </Flex>
    </GridItem>
  </Grid>
  <ConnectionPanel
    :open="open"
    title="Connection Panel"
    :teleports="teleports"
    :storages="storages"
    @close="open = false"
  />
</template>

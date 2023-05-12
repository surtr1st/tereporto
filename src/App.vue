<script lang="ts">
import { onMounted, ref, watch } from 'vue';
import { StorageResponse, TeleportResponse } from './types';
import { useStorage, useTeleport } from './server';
import { removeQuotes } from './helpers';
import ConnectionPanel from './components/ConnectionPanel.vue';
import Button from './components/Button.vue';
import ButtonGroup from './components/ButtonGroup.vue';
import Descriptive from './components/Descriptive.vue';
import StoragePanel from './components/StoragePanel.vue';
import TeleportPanel from './components/TeleportPanel.vue';
import GridItem from './components/GridItem.vue';
import Grid from './components/Grid.vue';
import Flex from './components/Flex.vue';
import DirectoryChooser from './components/DirectoryChooser.vue';
import ListItem from './components/ListItem.vue';
import List from './components/List.vue';
import Idol from './components/Idol.vue';
import TitleHeader from './components/TitleHeader.vue';
import FolderDestinationIcon from './components/Icon/FolderDestinationIcon.vue';
import FolderTransferIcon from './components/Icon/FolderTransferIcon.vue';
import TrashIcon from './components/Icon/TrashIcon.vue';
import FunctionalPanel from './components/FunctionalPanel.vue';
import Checkbox from './components/Checkbox.vue';
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
    .then((res) => {
      console.log(res);
      teleports.value = res;
    })
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
              :color="removeQuotes(teleport.color ?? '')"
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
              :color="removeQuotes(storage.color ?? '')"
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

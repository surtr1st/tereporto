<script lang="ts">
import ConnectionPanel from './components/ConnectionPanel.vue';
import Button from './components/Button.vue';
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
import Modal from './components/Modal.vue';
import ModalContent from './components/ModalContent.vue';
import GearIcon from './components/Icon/GearIcon.vue';
import FolderPathConnectIcon from './components/Icon/FolderPathConnectIcon.vue';
import MapMarkerIcon from './components/Icon/MapMarkerIcon.vue';
import DatabaseMarkerIcon from './components/Icon/DatabaseMarker.vue';
import SettingsPanel from './components/SettingsPanel.vue';
import { onMounted, ref, watch } from 'vue';
import { StorageResponse, Teleport, TeleportResponse, Storage } from './types';
import { useStorage, useTeleport, useDirectoryControl } from './server';
import { removeQuotes } from './helpers';
import { refresh, useToastification } from './globals';
</script>

<script setup lang="ts">
const openConnection = ref<boolean>(false);
const openSettings = ref<boolean>(false);
const openDirs = ref<boolean>(false);
const teleport = ref<string | string[]>('');
const storage = ref<string | string[]>('');
const teleports = ref<TeleportResponse[] | undefined>([]);
const unconnectedTeleports = ref<TeleportResponse[] | undefined>([]);
const storages = ref<StorageResponse[] | undefined>([]);
// const teleportDirs = ref<{ index: string; dirs: string[] }[]>([]);
const { getTeleports, createTeleport, removeTeleport } = useTeleport();
const { getStorages, createStorage, removeStorage } = useStorage();
const { openSelectedDir } = useDirectoryControl();
const { onSuccess, onError } = useToastification();

function retrieveTeleports() {
  getTeleports()
    .then((res) => {
      teleports.value = res;
      unconnectedTeleports.value = res?.filter((teleport) => !teleport.to);
    })
    .catch((e) => onError(`${e}`));
}

function retrieveStorages() {
  getStorages()
    .then((res) => (storages.value = res))
    .catch((e) => onError(`${e}`));
}

function createNewTeleport() {
  // Get the last element which is the folder name
  let name = '';
  const dir = `${teleport.value}`;
  if (dir.includes('\\')) name = dir.split('\\').at(-1) as string;
  else name = dir.split('/').at(-1) as string;

  const teleports: Teleport[] = [];

  if (Array.isArray(teleport.value)) {
    teleport.value.forEach((t) => {
      const teleportName = `${t}`.split('/').at(-1) as string;
      teleports.push({ name: teleportName, directory: t });
    });
  } else teleports.push({ name, directory: teleport.value });

  createTeleport(teleports)
    .then((rs) => onSuccess(rs))
    .catch((e) => onError(`${e}`));
}

function createNewStorage() {
  let name = '';
  const dir = `${storage.value}`;
  if (dir.includes('\\')) name = dir.split('\\').at(-1) as string;
  else name = dir.split('/').at(-1) as string;

  const storages: Storage[] = [];

  if (Array.isArray(storage.value)) {
    storage.value.forEach((s) => {
      const storageName = `${s}`.split('/').at(-1) as string;
      storages.push({ name: storageName, directory: s });
    });
  } else storages.push({ name, directory: storage.value });

  createStorage(storages)
    .then((rs) => onSuccess(rs))
    .catch((e) => onError(`${e}`));
}

function removeSelectedTeleport(index: string) {
  removeTeleport(removeQuotes(index))
    .then((rs) => {
      refresh.fetch = !refresh.fetch;
      onSuccess(rs);
    })
    .catch((e) => onError(`${e}`));
}

function removeSelectedStorage(index: string) {
  removeStorage(removeQuotes(index))
    .then((rs) => {
      refresh.fetch = !refresh.fetch;
      onSuccess(rs);
    })
    .catch((e) => onError(`${e}`));
}

function handleSelectedDirs(index: string) {
  teleports.value?.filter((t) => t.index === index).map((v) => v.directories);
  openDirs.value = true;
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

watch(
  () => refresh.fetch,
  () => {
    retrieveTeleports();
    retrieveStorages();
  },
);

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
          :title="$t('message.header.teleport')"
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
          :title="$t('message.header.storage')"
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
                @action="
                  openSelectedDir(removeQuotes(teleport.directories.join('')))
                "
              />
              <Button
                rounded
                :name="'teleport-trash-btn-' + index"
                color="danger"
                @click="removeSelectedTeleport(teleport.index)"
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
                @action="openSelectedDir(removeQuotes(storage.directory))"
              />
              <Button
                rounded
                :name="'storage-trash-btn-' + index"
                color="danger"
                @click="removeSelectedStorage(storage.index)"
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
          <DirectoryChooser
            :label="$t('message.feature.new_teleport')"
            name="teleport"
            v-model:select="teleport"
          >
            <MapMarkerIcon />
          </DirectoryChooser>
          <DirectoryChooser
            :label="$t('message.feature.new_storage')"
            name="storage"
            v-model:select="storage"
          >
            <DatabaseMarkerIcon />
          </DirectoryChooser>
          <Button
            name="scan-btn"
            :label="$t('message.feature.scan_teleport')"
            color="darker"
            disabled
            rounded
            larger
          />
          <Button
            name="connection-btn"
            :label="$t('message.feature.connection')"
            color="danger"
            rounded
            larger
            icon
            @click="openConnection = true"
          >
            <FolderPathConnectIcon />
          </Button>
          <Button
            name="settings-btn"
            color="darker"
            rounded
            icon
            @click="openSettings = true"
          >
            <GearIcon />
          </Button>
        </FunctionalPanel>
      </Flex>
    </GridItem>
  </Grid>
  <Modal
    :open="openDirs"
    title="Directories"
    @close="openDirs = false"
  >
    <ModalContent>
      <Flex>
        <Descriptive description="dir.dirs" />
      </Flex>
    </ModalContent>
  </Modal>
  <ConnectionPanel
    :open="openConnection"
    :title="$t('message.panel.connection.title')"
    :teleports="unconnectedTeleports"
    :storages="storages"
    @close="openConnection = false"
  />
  <SettingsPanel
    :open="openSettings"
    :title="$t('message.panel.settings.title')"
    @close="openSettings = false"
  />
</template>

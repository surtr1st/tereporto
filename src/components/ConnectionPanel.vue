<script setup lang="ts">
import Flex from './Flex.vue';
import Grid from './Grid.vue';
import GridItem from './GridItem.vue';
import Button from './Button.vue';
import RadioButton from './RadioButton.vue';
import Modal from './Modal.vue';
import ModalContent from './ModalContent.vue';
import ModalFooter from './ModalFooter.vue';
import { StorageResponse, TeleportResponse } from '../types';
import { ref } from 'vue';
import { removeQuotes, generateRandomHexColor } from '../helpers';
import { useStorage, useTeleport } from '../server';
import { refresh } from '../globals';

interface IConnectionPanel {
  open?: boolean;
  title?: string;
  onClose?: () => void | Promise<void>;
  teleports?: TeleportResponse[];
  storages?: StorageResponse[];
}
const { onClose } = defineProps<IConnectionPanel>();

const { updateTeleport } = useTeleport();
const { updateStorage } = useStorage();

const teleportIndex = ref<string>('');
const storageIndex = ref<string>('');

function connect() {
  if (!teleportIndex.value || !storageIndex.value) return;
  if (teleportIndex.value.length === 0 || storageIndex.value.length === 0)
    return;

  const color = generateRandomHexColor();

  Promise.all([
    updateTeleport({
      filename: `${teleportIndex.value}.toml`,
      target: { field: 'to', value: storageIndex.value },
    }),
    updateStorage({
      filename: `${storageIndex.value}.toml`,
      target: { field: 'constraint', value: teleportIndex.value },
    }),
    updateTeleport({
      filename: `${teleportIndex.value}.toml`,
      target: { field: 'color', value: color },
    }),
    updateStorage({
      filename: `${storageIndex.value}.toml`,
      target: { field: 'color', value: color },
    }),
  ])
    .then(() => {
      onClose!();
      refresh.fetch = !refresh.fetch;
    })
    .catch((e) => console.log(e));
}
</script>

<template>
  <Modal
    :open="open"
    :title="title"
    @close="onClose"
  >
    <ModalContent>
      <Grid type="panel">
        <GridItem position="main">
          <Flex
            justify-content="center"
            align-items="flex-start"
            column
          >
            <RadioButton
              v-for="(teleport, index) in teleports"
              :key="index"
              :id="'teleport-radio-' + index"
              name="teleport-radio"
              :label="removeQuotes(teleport.name)"
              :value="removeQuotes(teleport.index)"
              v-model:selected="teleportIndex"
            />
          </Flex>
        </GridItem>
        <GridItem position="aside">
          <Flex
            justify-content="center"
            align-items="flex-start"
            column
          >
            <RadioButton
              v-for="(storage, index) in storages"
              :key="index"
              :id="'storage-radio-' + index"
              name="storage-radio"
              :label="removeQuotes(storage.name)"
              :value="removeQuotes(storage.index)"
              :checked="removeQuotes(storage.index) === storage.index"
              v-model:selected="storageIndex"
            />
          </Flex>
        </GridItem>
      </Grid>
    </ModalContent>
    <ModalFooter>
      <Flex
        align-items="center"
        justify-content="center"
      >
        <Button
          name="connect-btn"
          color="neutral"
          label="Connect"
          rounded
          larger
          @click="connect()"
        />
      </Flex>
    </ModalFooter>
  </Modal>
</template>

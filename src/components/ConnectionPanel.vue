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
import { removeQuotes } from '../helpers';

interface IConnectionPanel {
  open?: boolean;
  title?: string;
  onClose?: () => void | Promise<void>;
  teleports?: TeleportResponse[];
  storages?: StorageResponse[];
}
defineProps<IConnectionPanel>();

const teleportIndex = ref<string>('');
const storageIndex = ref<string>('');
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
              :checked="removeQuotes(storage.index) === storageIndex"
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
        />
      </Flex>
    </ModalFooter>
  </Modal>
</template>

<script setup lang="ts">
import Flex from './Flex.vue';
import Button from './Button.vue';
import RadioButton from './RadioButton.vue';
import Modal from './Modal.vue';
import ModalContent from './ModalContent.vue';
import ModalFooter from './ModalFooter.vue';
import { reactive, ref } from 'vue';
import { refresh } from '../globals';
import Checkbox from './Checkbox.vue';
import PlaceHolder from './PlaceHolder.vue';

interface IConnectionPanel {
  open?: boolean;
  title?: string;
  onClose?: () => void | Promise<void>;
}
const { onClose } = defineProps<IConnectionPanel>();

const langs = [
  { name: 'VI', value: 'vi' },
  { name: 'EN', value: 'en' },
  { name: 'JP', value: 'ja' },
];

const settings = reactive({
  lang: '',
  autoScan: false,
});

function setOptions() {}
</script>

<template>
  <Modal
    :open="open"
    :title="title"
    @close="onClose"
  >
    <ModalContent>
      <Flex
        justify-content="center"
        align-items="center"
        :margin-bottom="25"
        :margin-top="25"
      >
        <Flex
          align-items="flex-start"
          column
          :width="320"
        >
          <PlaceHolder :title="$t('message.panel.settings.auto.title')">
            <Checkbox :label="$t('message.panel.settings.auto.scan')" />
          </PlaceHolder>
          <PlaceHolder :title="$t('message.panel.settings.lang.title')">
            <Flex
              justify-content="space-between"
              align-items="flex-start"
              :width="320"
              :gap="27"
            >
              <RadioButton
                v-for="(lang, index) in langs"
                :key="index"
                :id="'teleport-radio-' + index"
                name="teleport-radio"
                :label="lang.name"
                :value="lang.value"
                v-model:selected="settings.lang"
              />
            </Flex>
          </PlaceHolder>
          <PlaceHolder :title="$t('message.panel.settings.system.title')">
          </PlaceHolder>
        </Flex>
      </Flex>
    </ModalContent>
    <ModalFooter>
      <Flex
        align-items="center"
        justify-content="center"
      >
        <Button
          :label="$t('message.feature.save')"
          name="save-btn"
          color="neutral"
          larger
          rounded
        />
      </Flex>
    </ModalFooter>
  </Modal>
</template>

<script setup lang="ts">
import {ref} from "vue";
import CalendarField from "./CalendarField.vue";
import ResourcePopup from "./ResourcePopup.vue";
import { computed } from "vue";
import { SubmitEventPromise } from "vuetify";
import { watch } from "vue";
    const props = defineProps([
      'item_to_edit'
    ])
    const dialog = defineModel({type: Boolean})
    const resource_dialog = ref(false)
    const emit = defineEmits(["sendValues", "updateOpenDialogBool"])
    const field = ref({
      priority: "",
    })
    

    const values = ref({
      id: NaN, // may cause error
      name: "",
      parent: NaN,
      priority: computed(()=> {return field.value.priority == "" ? 100 : parseInt(field.value.priority)}),
      est_time: "",
      resource: "",
      resource_link: "",
      resource_type: "",
      start_date: "",
      end_date: "",
      availability: "",
      completed: false,
      description: "",
    })

    watch(dialog, (val) => {
        if (val == true){
            if(props.item_to_edit != undefined){
                
                field.value.priority = props.item_to_edit.priority

                values.value.id = props.item_to_edit.id
                values.value.name = props.item_to_edit.name
                values.value.parent = props.item_to_edit.parent
                values.value.est_time = props.item_to_edit.est_time
                values.value.resource = props.item_to_edit.resource
                values.value.resource_link = props.item_to_edit.resource_link
                values.value.resource_type = props.item_to_edit.resource_type
                values.value.start_date = props.item_to_edit.start_date
                values.value.end_date = props.item_to_edit.end_date
                values.value.availability = props.item_to_edit.availability
                values.value.completed = props.item_to_edit.completed
                values.value.description = props.item_to_edit.description
            }
            else{
                field.value.priority = ""

                values.value.name = ""
                values.value.parent = NaN
                values.value.est_time = ""
                values.value.resource = ""
                values.value.resource_type = ""
                values.value.resource_link = ""
                values.value.start_date = ""
                values.value.end_date = ""
                values.value.availability = ""
                values.value.completed = false
                values.value.description = ""
            }
        }
      
    })

    const rules = ref({
      isSmallInt: (value: string) => {
        if ((!isNaN(parseInt(value, 10)) && parseInt(value) >= 0 && parseInt(value) <= 100) || value == "") return true
        else return `Must be an integer within 0-100`
      },
      required: (value: string) => {
        if (value) return true
        else return "Required"
      }
    })

    async function submit(event: SubmitEventPromise){
      const result: any = await event
      if(result.valid){
        emit("sendValues", values.value)
        dialog.value=false
      }
    }
    function updateAddress(address_arr: Array<string>){
      values.value.resource_type = address_arr[0]
      values.value.resource_link = address_arr[1]
    }
  
</script>
<template class="mr-5 text-center">
    <v-dialog
      v-model="dialog"
      max-width="1000"
    >
        <template v-if="props.item_to_edit == undefined" v-slot:activator="{ props: activatorProps }">
        <v-btn
          class="bg-primary mr-5"
          prepend-icon="mdi-plus-circle"
          text="Add Item"
          v-bind="activatorProps"
        ></v-btn>
      </template>

      <v-form validate-on="submit" @submit.prevent="submit">
        <v-card
          prepend-icon="mdi-plus-circle"
          title="Add Item"
        >
          <v-card-text>
            <v-row dense>
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Name*"
                  v-model="values.name"
                  :rules="[rules.required]"
                  maxlength="30"
                  aria-autocomplete="none"
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Priority"
                  v-model="field.priority"
                  hint="Will default to 100 if left empty"
                  default="100"
                  persistent-hint
                  :rules="[rules.isSmallInt]"
                  maxlength="3"            
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  v-model="values.est_time"
                  hint="Estimated time the item will take"
                  label="Est Time"
                  suffix="hrs"
                  maxlength="8"
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  v-model="values.resource"
                  append-inner-icon="mdi-link"
                  @click:append-inner="resource_dialog=true"
                  maxlength="15"
                  hint="If a link exists while this textfield is empty, it will default to 'link'"
                  persistent-hint
                  counter
                  label="Resource"
                  aria-autocomplete="none"
                ></v-text-field>
              </v-col>
              <ResourcePopup v-model="resource_dialog" :address_str="values.resource_link" :address_type="values.resource_type" @add-address="updateAddress"/>

              <v-col
                cols="12"
                md="4"
                sm="6"
              >
              <CalendarField :date_str="values.start_date" name="Start Date" @send-date="(v) => values.start_date = v" />
              </v-col>
  
              <v-col
                cols="12"
                sm="4"
              >
                <CalendarField :date_str="values.end_date" name="End Date" @send-date="(v) => values.end_date = v"/>
              </v-col>

              <v-col
                cols="12"
                md="4"
                sm="6"
              >
              <CalendarField :date_str="values.availability" name="Availability" @send-date="(v) => values.availability = v"/>
              </v-col>

              <v-col
                cols="12"
                sm="4"
              >
                <v-checkbox-btn
                  v-model="values.completed"
                  label="Completed?"
                ></v-checkbox-btn>
              </v-col>
  
              <v-col
                cols="12"
                sm="12"
              >
                <v-textarea
                  v-model="values.description"
                  label="Description"
                  maxlength="500"
                ></v-textarea>
              </v-col>
            </v-row>
  
            <small class="text-caption text-medium-emphasis">*indicates required field</small>
          </v-card-text>
  
          <v-divider></v-divider>
  
          <v-card-actions>
            <v-btn
              class="bg-primary"
              :text="props.item_to_edit == undefined ? 'Save' : 'Update'"
              variant="tonal"
              type="submit"
            ></v-btn>
            
            <v-spacer></v-spacer>
  
            <v-btn
              class="bg-grey"
              text="Discard"
              variant="plain"
              @click= "dialog = false"
            ></v-btn>
            
            <!-- <v-btn
              text="Add Another"
              prepend-icon="mdi-plus-circle"
              variant="plain"
              @click="onSubmit"
            ></v-btn> -->

          </v-card-actions>
        </v-card>
      </v-form>
    </v-dialog>
  </template>
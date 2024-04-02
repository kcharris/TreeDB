<script setup>
import {ref, reactive} from "vue";
import CalendarField from "./CalendarField.vue";
    // defineEmits(["sendValues"])
    const dialog = ref(false)
    // const curr_priority = ref(100)
    const values = reactive({
      name: "",
      priority: "",
      estTime: "",
      resource: "",
      startDate: "",
      endDate: "",
      availability: "",
      completed: "",
      description: "",
    })
    const rules = ref({
      required: value => {
        if (!isNaN(parseInt(value, 10)) && value >= 0 && value <= 100) return true
        else return `Must be an integer within 0-100`
      }
    })

    function onSubmit(){
      dialog=false
      // $emit("sendValues", values)
    }
  
</script>
<template>
    <div class="pa-4 text-center">
      <v-dialog
        v-model="dialog"
        max-width="1000"
      >
        <template v-slot:activator="{ props: activatorProps }">
          <v-btn
            class="text-none font-weight-regular"
            prepend-icon="mdi-plus-circle"
            text="Add Item"
            variant="tonal"
            v-bind="activatorProps"
          ></v-btn>
        </template>
  
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
                  required
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Priority"
                  hint="Will default to 100 if left empty"
                  persistent-hint
                  :rules="[rules.required]"                 
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  hint="Estimated time the item will take"
                  label="Est Time"
                  suffix="hrs"
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  hint="Will try to convert to link"
                  label="Resource"
                  required
                ></v-text-field>
              </v-col>

              <v-col
                cols="12"
                md="4"
                sm="6"
              >
              <!-- @sendDate="(v) => values.startDate = v" -->
              <CalendarField name="Start Date" />
              </v-col>
  
              <v-col
                cols="12"
                sm="4"
              >
                <CalendarField name="End Date"/>
              </v-col>

              <v-col
                cols="12"
                md="4"
                sm="6"
              >
              <CalendarField name="Availability"/>
              </v-col>

              <v-col
                cols="12"
                sm="4"
              >
                <v-checkbox-btn
                  label="Completed?"
                ></v-checkbox-btn>
              </v-col>
  
              <v-col
                cols="12"
                sm="12"
              >
                <v-textarea
                  label="Description"
                ></v-textarea>
              </v-col>
            </v-row>
  
            <small class="text-caption text-medium-emphasis">*indicates required field</small>
          </v-card-text>
  
          <v-divider></v-divider>
  
          <v-card-actions>
            <v-btn
              color="primary"
              text="Save"
              variant="tonal"
              @click="dialog = false"
            ></v-btn>
            
            <v-spacer></v-spacer>
  
            <v-btn
              text="Discard"
              variant="plain"
              @click="dialog = false"
            ></v-btn>
            
            <v-btn
              text="Add Another"
              prepend-icon="mdi-plus-circle"
              variant="plain"
              @click="onSubmit"
            ></v-btn>

          </v-card-actions>
        </v-card>
      </v-dialog>
    </div>
  </template>
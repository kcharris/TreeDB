<script setup>
import {ref} from "vue";
    const dialog = ref(false)
    // const curr_priority = ref(100)
    const rules = ref({
      required: value => {
        if (!value.trim()) return true 
        if (!isNaN(parseInt(value, 10)) && value >= 0 && value <= 100) return true
        else return `Must be an integer within 0-100`
      }
    })
  
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
                  required
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  hint="Will default to 100 if left empty"
                  persistent-hint
                  :rules="[rules.required]"
                  label="Priority*"
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Last name*"
                  required
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Email*"
                  required
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Password*"
                  type="password"
                  required
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                md="4"
                sm="6"
              >
                <v-text-field
                  label="Confirm Password*"
                  type="password"
                  required
                ></v-text-field>
              </v-col>
  
              <v-col
                cols="12"
                sm="6"
              >
                <v-select
                  :items="['0-17', '18-29', '30-54', '54+']"
                  label="Age*"
                  required
                ></v-select>
              </v-col>
  
              <v-col
                cols="12"
                sm="6"
              >
                <v-autocomplete
                  :items="['Skiing', 'Ice hockey', 'Soccer', 'Basketball', 'Hockey', 'Reading', 'Writing', 'Coding', 'Basejump']"
                  label="Interests"
                  auto-select-first
                  multiple
                ></v-autocomplete>
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
              @click="dialog = false"
            ></v-btn>

          </v-card-actions>
        </v-card>
      </v-dialog>
    </div>
  </template>
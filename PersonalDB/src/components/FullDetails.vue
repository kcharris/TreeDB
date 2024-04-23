<script>
    export default{
        data: () =>({
            item_name: "Cleaning"
        })
    }
</script>
<template >
    <div class="mx-4 mt-2">
        <div class="text-h5">{{ "Name: " + item_name }}</div>
        <div class="d-flex">
            <div class="text-md mr-5">Priority: 80</div>
            <div>               </div>
            <div class="text-md">Completed?: False</div>
        </div>
        <div class="text-md">Link to resource: link</div>
        <div class="text-md">Est Time: 2hrs</div>
        <div class="text-md">Available: 04/02/2024</div>
        <div class="d-flex">
            <div class="text-md mr-5">Start Time: 04/02/2024</div>
            <div>               </div>
            <div class="text-md">End Time: 04/02/2024</div>
        </div>
        <div class="text-md text-truncate text-nowrap">Description: This is an example of the fully available items details. This Cleaning item serves as a categorical item that will contain other items.</div>
    </div>
</template>
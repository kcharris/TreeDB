export type Item = {
    name: string,
    id?: number,
    parent_id?: number,
    priority?: number,
    est_time?: number,
    resource?: string,
    resource_type?: string,
    resource_link?: string,
    start_date?: string,
    end_date?: string,
    availability?: string,
    completed?: boolean,
    description?: string
}

export type Tag = {
    id?: number,
    name: string,
}

export type ItemTag = {
    item_id: number,
    tag_id: number
}
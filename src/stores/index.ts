import type { ITempTimestamp } from "$lib";
import { writable } from "svelte/store";

let timeline: { [device: string]: ITempTimestamp[] } = {};
export const timelinestore = writable(timeline);
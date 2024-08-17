import moment from "moment";

export interface ITempTimestamp {
    timestamp: string;
    temp: number;
}

export function temp_timestamp(temp: number): ITempTimestamp {
    return {
        timestamp: moment().format("HH:mm:ss"),
        temp,
    }
}
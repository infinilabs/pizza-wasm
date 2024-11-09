/* tslint:disable */
/* eslint-disable */
/**
*/
export class Pizza {
  free(): void;
/**
* @returns {Pizza}
*/
  static new(): Pizza;
/**
* @param {string} data
* @returns {boolean}
*/
  load_json_objects_array(data: string): boolean;
/**
* @param {string} query_string
* @returns {any}
*/
  search_by_query_string(query_string: string): any;
/**
* @param {string} default_field
* @param {string} query_string
* @param {string} operator
* @param {number} from
* @param {number} size
* @param {boolean} explain
* @returns {any}
*/
  advanced_search_by_query_string(default_field: string, query_string: string, operator: string, from: number, size: number, explain: boolean): any;
}

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
* @param {string} query_string
* @param {string} operator
* @returns {any}
*/
  search_by_query_string_with_default_operator(query_string: string, operator: string): any;
}

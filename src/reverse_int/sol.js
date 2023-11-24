/**
 * @param {number} x
 * @return {number}
 */
const reverse = (x) => {
  if (x > 0) return parseInt(x.toString().split('').reverse().join('')) > Math.pow(2, 31) - 1 ? 0 : parseInt(x.toString().split('').reverse().join(''))
  else return -parseInt(x.toString().split('').reverse().join('')) < -(Math.pow(2, 31)) ? 0 : -parseInt(x.toString().split('').reverse().join(''))   
};
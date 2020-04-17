function getAll(selector) {
  return Array.prototype.slice.call(document.querySelectorAll(selector), 0);
}

function registerDelete() {
  var $foodDeleteButtons = getAll('.food-delete');

  if ($foodDeleteButtons.length > 0) {
    $foodDeleteButtons.forEach(function ($el) {
      $el.addEventListener('click', function () {
        var target = document.getElementById($el.dataset.target);
        document.getElementById('food-items').removeChild(target);
      });
    });
  }
}

var $foodItemCount = 1;

document.getElementById('add-food').addEventListener('click', function () {
  var foodItemTemplate = document.getElementById('food-item');
  var foodItem = foodItemTemplate.content.cloneNode(true);

  foodItem.getElementById('item').id = `food-item-${$foodItemCount}`
  foodItem.getElementById('food-input').name = `food-item-${$foodItemCount}`
  foodItem.getElementById('food-delete').dataset.target = `food-item-${$foodItemCount}`

  document.getElementById('food-items').appendChild(foodItem);

  registerDelete();

  $foodItemCount++;
});

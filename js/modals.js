function getAll(selector) {
  return Array.prototype.slice.call(document.querySelectorAll(selector), 0);
}

var rootEl = document.documentElement;
var $modals = getAll('.modal');
var $modalButtons = getAll('.modal-button');
var $modalCloses = getAll('.modal-background, .modal-close, .modal-card-head .delete, .cancel');

if ($modalButtons.length > 0) {
 $modalButtons.forEach(function ($el) {
   $el.addEventListener('click', function () {
     var target = $el.dataset.target;
     openModal(target);
   });
 });
}

if ($modalCloses.length > 0) {
 $modalCloses.forEach(function ($el) {
   $el.addEventListener('click', function () {
     var target = $el.dataset.target;
     closeModal(target);
   });
 });
}

function openModal(target) {
 var $target = document.getElementById(target);
 rootEl.classList.add('is-clipped');
 $target.classList.add('is-active');
}

function closeModal(target) {
 var $target = document.getElementById(target);
 rootEl.classList.remove('is-clipped');
 $target.classList.remove('is-active');
}

var $dropdowns = getAll('.dropdown:not(.is-hoverable)');

if ($dropdowns.length > 0) {
 $dropdowns.forEach(function ($el) {
   $el.addEventListener('click', function (event) {
     event.stopPropagation();
     $el.classList.toggle('is-active');
   });
 });

 document.addEventListener('click', function (event) {
   closeDropdowns();
 });
}

// document.addEventListener('keydown', function (event) {
//  var e = event || window.event;
//  if (e.keyCode === 27) {
//    closeModal();
//    closeDropdowns();
//  }
// });

//= require 'jquery-2.1.0.min'
$(function(){
    $('#main>h3').each(function(){
        $(this).addClass('fold_button');
        $(this).after("<pre class='fold_begin'></pre>");
        $(this).nextAll('h1,h2,h3').first().before("<pre class='fold_end'></pre>");
    });
});
$(function(){
    $('.fold_button:not(.fold_button_processed)').each(function(){
         $(this).addClass('fold_button_processed')
         $(this).prepend('<span class="fold-plus">+</span> ');
         $(this).nextAll('.fold_begin').first().nextUntil('.fold_end').wrapAll("<div class='fold' />");
         $(this).nextAll('.fold').first().toggle(false);
         $(this).on('click', function(){
             const elem=$(this).find('.fold-plus');
             elem.text(elem.text() == '+' ? '-' : '+');
             $(this).nextAll('.fold').first().toggle();
         });
    });
});

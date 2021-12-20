from django.urls import path
from paypal_pay import views

# Urls for user login/registration/logout
urlpatterns = [
    # Below is only an example
    path('create-order', views.create_order_view, name='create-order'),
    path('capture-order', views.capture_order_view, name='capture-order'),
    path('orders', views.orders_view, name='orders'),
    path('teste', views.teste, name='teste'),

]
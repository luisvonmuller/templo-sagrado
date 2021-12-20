from django.urls import path
from stripe_pay import views

# Urls for user login/registration/logout
urlpatterns = [
    path('source', views.source, name='source'),
    path('charge', views.charge, name='charge'),
    path('teste', views.teste, name='teste'),
]

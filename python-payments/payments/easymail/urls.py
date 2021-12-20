from django.urls import path
from easymail import views

# Urls for user login/registration/logout
urlpatterns = [
    path('sendemail', views.send_email, name='sendemail'),
]
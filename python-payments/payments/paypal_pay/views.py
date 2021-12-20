from django.shortcuts import render
from django.http import HttpResponse, JsonResponse

import json, requests, pprint
from paypal_pay.paypal_orders import CreateOrder, CaptureOrder

# Disable CSRF check for view
from django.views.decorators.csrf import csrf_exempt
# Create your views here.
URL_PAYPAL_CREATE_SALE = 'https://templo-sagrado.com/new-paypal-sale'
URL_PAYPAL_UPDATE_SALE = 'https://templo-sagrado.com/update-paypal-sale'


@csrf_exempt
def create_order_view(request):
    print('Recebi um Create Order')
    if request.method == 'POST':
        # read and load the webhook
        webhook = request.read()
        raw_data = json.loads(webhook)
        print(raw_data)
        response = CreateOrder().create_order(raw_data)
        print(response.result.id)
        ###### Envia a compra para registro no banco de dados ######
        try:
            post_data = {
                'user_id': int(raw_data['purchase_units'][0]['reference_user_id']),
                'product_id': int(raw_data['purchase_units'][0]['reference_id']),
                'product_value': float(response.result.purchase_units[0].amount.value),
                'reference_code': response.result.id
            }
            newHeaders = {
                'Content-type': 'application/json', 
                'Accept': 'text/plain'
            }
            print(post_data)
            post = requests.post(URL_PAYPAL_CREATE_SALE, data=json.dumps(post_data), headers = newHeaders)
        except:
            print("An exception occurred")
    return JsonResponse({'id':response.result.id})

@csrf_exempt
def capture_order_view(request):
    print('Recebi um Capture Order')
    if request.method == 'POST':
        # read and load the webhook
        webhook = request.read()
        raw_data = json.loads(webhook)
        print(raw_data)
        response = CaptureOrder().capture_order(raw_data['orderID'])
        #print(response.result.purchase_units.reference_id)
        ###### Envia a compra para registro no banco de dados ######
        try:
            post_data = {
                'reference_code': response.result.id
            }
            newHeaders = {
                'Content-type': 'application/json', 
                'Accept': 'text/plain'
            }
            print(post_data)
            post = requests.post(URL_PAYPAL_UPDATE_SALE, data=json.dumps(post_data), headers = newHeaders)
        except:
            print("An exception occurred")
    return JsonResponse({'details':"OK"})

@csrf_exempt
def orders_view(request):
    print('Recebi um WeebHook')
    if request.method == 'POST':
        # read and load the webhook
        webhook = request.read()
        raw_data = json.loads(webhook)
        #print(raw_data)
    return HttpResponse("OK")


@csrf_exempt
def teste(request):
    return render(request, "teste.html")

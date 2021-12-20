from django.shortcuts import render
from django.http import HttpResponse

import json, requests, pprint

# Disable CSRF check for view
from django.views.decorators.csrf import csrf_exempt

# Stripe stuff
import stripe

URL_STRIPE_UPDATE_SALE = 'https://templo-sagrado.com/admin/sales/update-stripe-sale'
# Create your views here.

def teste(request):
    print('Recebi um request')
    return HttpResponse("OK")

@csrf_exempt
def source(request):
    print('Recebi um source')
    if request.method == 'POST':
        # read and load the webhook
        webhook = request.read()
        raw_data = json.loads(webhook)
        data = raw_data['data']['object']
        stripe.api_key = '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'
        try:
            status = data['status']
            
            if status == 'pending':
                print('Ainda pendente')
            
            elif status == 'canceled':
                print('Transação cancelada')
            
            elif status == 'chargeable':
                print('Já pode ser requisitada')
                try:
                    charge = stripe.Charge.create(
                        amount=data['amount'],
                        currency=data['currency'],
                        source=data['id'],
                    )
                    print('Consegui requisitar o charge')
                except:
                    print("An exception occurred when creating a charge")
            
            elif status == 'consumed':
                print('Transação consumida')
            
            elif status == 'failed':
                print('Transação falhou')

        except:
            print("An exception occurred")
        
        
        #resp = {
        #    'user_groups':UserGroups(request),
        #    }
        #requests.post(URL_STRIPE_UPDATE_SALE,'teste')
    return HttpResponse("OK")
        

@csrf_exempt
def charge(request):
    print('Recebi um charge')
    if request.method == 'POST':
        # read and load the webhook
        webhook = request.read()
        raw_data = json.loads(webhook)
        data = raw_data['data']['object']
        pp = pprint.PrettyPrinter(indent=2)
        try:
            status = data['status']
            if status == 'pending':
                print('Ainda pendente')
            
            elif status == 'failed':
                print('Transação falhou')
    
            elif status == 'succeeded':
                
                data['source']['metadata']['product_id']
                post_data = {
                    'stripe_source':data['source']['id'],
                    'status':'1',
                    'product_id':data['source']['metadata']['product_id'],
                }
                pp.pprint(post_data)
                newHeaders = {'Content-type': 'application/json', 'Accept': 'text/plain'}
                response = requests.post(URL_STRIPE_UPDATE_SALE, data=json.dumps(post_data), headers = newHeaders)
                print("Status code: ", response.status_code)
        except:
            print("An exception occurred")
    
    return HttpResponse("OK")

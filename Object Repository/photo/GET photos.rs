<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET photos</name>
   <tag></tag>
   <elementGuidId>3658f186-b41c-416e-a681-53e9aa7ef97d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/photos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


//verifikasi status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//verifikasi data ke 1
WS.verifyElementPropertyValue(response, '[0].albumId', '1')
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].title', 'accusamus beatae ad facilis cum similique qui sunt')
WS.verifyElementPropertyValue(response, '[0].url', 'https://via.placeholder.com/600/92c952')
WS.verifyElementPropertyValue(response, '[0].thumbnailUrl', 'https://via.placeholder.com/150/92c952')

//verifikasi data ke 2
WS.verifyElementPropertyValue(response, '[1].albumId', '1')
WS.verifyElementPropertyValue(response, '[1].id', '2')
WS.verifyElementPropertyValue(response, '[1].title', 'reprehenderit est deserunt velit ipsam')
WS.verifyElementPropertyValue(response, '[1].url', 'https://via.placeholder.com/600/771796')
WS.verifyElementPropertyValue(response, '[1].thumbnailUrl', 'https://via.placeholder.com/150/771796')

//verifikasi data ke 3
WS.verifyElementPropertyValue(response, '[2].albumId', '1')
WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].title', 'officia porro iure quia iusto qui ipsa ut modi')
WS.verifyElementPropertyValue(response, '[2].url', 'https://via.placeholder.com/600/24f355')
WS.verifyElementPropertyValue(response, '[2].thumbnailUrl', 'https://via.placeholder.com/150/24f355')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

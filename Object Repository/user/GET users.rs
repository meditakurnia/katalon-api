<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET users</name>
   <tag></tag>
   <elementGuidId>04812248-c19a-463c-bb80-27eb0adf4909</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
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

//verifikasi status code 200 OK
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//verifikasi data ke 1
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].name', 'Leanne Graham')
WS.verifyElementPropertyValue(response, '[0].username', 'Bret')
WS.verifyElementPropertyValue(response, '[0].email', 'Sincere@april.biz')
WS.verifyElementPropertyValue(response, '[0].address.street', 'Kulas Light')
WS.verifyElementPropertyValue(response, '[0].address.suite', 'Apt. 556')
WS.verifyElementPropertyValue(response, '[0].address.city', 'Gwenborough')
WS.verifyElementPropertyValue(response, '[0].address.zipcode', '92998-3874')
WS.verifyElementPropertyValue(response, '[0].address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, '[0].address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, '[0].phone', '1-770-736-8031 x56442')
WS.verifyElementPropertyValue(response, '[0].website', 'hildegard.org')
WS.verifyElementPropertyValue(response, '[0].company.name', 'Romaguera-Crona')
WS.verifyElementPropertyValue(response, '[0].company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, '[0].company.bs', 'harness real-time e-markets')

//verifikasi data ke 2
WS.verifyElementPropertyValue(response, '[1].id', '2')
WS.verifyElementPropertyValue(response, '[1].name', 'Ervin Howell')
WS.verifyElementPropertyValue(response, '[1].username', 'Antonette')
WS.verifyElementPropertyValue(response, '[1].email', 'Shanna@melissa.tv')
WS.verifyElementPropertyValue(response, '[1].address.street', 'Victor Plains')
WS.verifyElementPropertyValue(response, '[1].address.suite', 'Suite 879')
WS.verifyElementPropertyValue(response, '[1].address.city', 'Wisokyburgh')
WS.verifyElementPropertyValue(response, '[1].address.zipcode', '90566-7771')
WS.verifyElementPropertyValue(response, '[1].address.geo.lat', '-43.9509')
WS.verifyElementPropertyValue(response, '[1].address.geo.lng', '-34.4618')
WS.verifyElementPropertyValue(response, '[1].phone', '010-692-6593 x09125')
WS.verifyElementPropertyValue(response, '[1].website', 'anastasia.net')
WS.verifyElementPropertyValue(response, '[1].company.name', 'Deckow-Crist')
WS.verifyElementPropertyValue(response, '[1].company.catchPhrase', 'Proactive didactic contingency')
WS.verifyElementPropertyValue(response, '[1].company.bs', 'synergize scalable supply-chains')

//verifikasi data ke 3
WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].name', 'Clementine Bauch')
WS.verifyElementPropertyValue(response, '[2].username', 'Samantha')
WS.verifyElementPropertyValue(response, '[2].email', 'Nathan@yesenia.net')
WS.verifyElementPropertyValue(response, '[2].address.street', 'Douglas Extension')
WS.verifyElementPropertyValue(response, '[2].address.suite', 'Suite 847')
WS.verifyElementPropertyValue(response, '[2].address.city', 'McKenziehaven')
WS.verifyElementPropertyValue(response, '[2].address.zipcode', '59590-4157')
WS.verifyElementPropertyValue(response, '[2].address.geo.lat', '-68.6102')
WS.verifyElementPropertyValue(response, '[2].address.geo.lng', '-47.0653')
WS.verifyElementPropertyValue(response, '[2].phone', '1-463-123-4447')
WS.verifyElementPropertyValue(response, '[2].website', 'ramiro.info')
WS.verifyElementPropertyValue(response, '[2].company.name', 'Romaguera-Jacobson')
WS.verifyElementPropertyValue(response, '[2].company.catchPhrase', 'Face to face bifurcated interface')
WS.verifyElementPropertyValue(response, '[2].company.bs', 'e-enable strategic applications')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

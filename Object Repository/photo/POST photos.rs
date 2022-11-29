<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST photos</name>
   <tag></tag>
   <elementGuidId>f6c9f888-1a62-451b-a06a-71dd3df80309</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{ \&quot;data\&quot; : \n [\n   {\n    \&quot;albumId\&quot;: 1,\n    \&quot;title\&quot;: \&quot;ini photo\&quot;,\n    \&quot;url\&quot;: \&quot;https://google.com\&quot;,\n    \&quot;thumbnailUrl\&quot;: \&quot;https://google.com\&quot;\n   },\n   {\n    \&quot;albumId\&quot;: 1,\n    \&quot;title\&quot;: \&quot;ini photoo\&quot;,\n    \&quot;url\&quot;: \&quot;https://googlee.com\&quot;,\n    \&quot;thumbnailUrl\&quot;: \&quot;https://googlee.com\&quot;\n   }\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bb6ad80f-1448-454a-ac62-df35cc3d93b1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
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
WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)

////verifikasi data yang diinput
//WS.verifyElementPropertyValue(response, 'albumId', '1')
//WS.verifyElementPropertyValue(response, 'title', 'ini photo')
//WS.verifyElementPropertyValue(response, 'url', 'https://google.com')
//WS.verifyElementPropertyValue(response, 'thumbnailUrl', 'https://google.com')
//WS.verifyElementPropertyValue(response, 'id', '5001')

//verifikasi data yang diinput

WS.verifyElementPropertyValue(response, 'data[0].albumId', '1')
WS.verifyElementPropertyValue(response, 'data[0].title', 'ini photo')
WS.verifyElementPropertyValue(response, 'data[0].url', 'https://google.com')
WS.verifyElementPropertyValue(response, 'data[0].thumbnailUrl', 'https://google.com')
WS.verifyElementPropertyValue(response, 'data[1].albumId', '1')
WS.verifyElementPropertyValue(response, 'data[1].title', 'ini photoo')
WS.verifyElementPropertyValue(response, 'data[1].url', 'https://googlee.com')
WS.verifyElementPropertyValue(response, 'data[1].thumbnailUrl', 'https://googlee.com')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

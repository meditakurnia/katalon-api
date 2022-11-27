<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST users</name>
   <tag></tag>
   <elementGuidId>cf17cc9a-40fe-4caa-ba0b-8dd9ffe47ac3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Putri Titian\&quot;,\n    \&quot;username\&quot;: \&quot;putri_titian\&quot;,\n    \&quot;email\&quot;: \&quot;putritian@gmail.com\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Jakarta\&quot;,\n      \&quot;suite\&quot;: \&quot;Tebet. 556\&quot;,\n      \&quot;city\&quot;: \&quot;Jaksel\&quot;,\n      \&quot;zipcode\&quot;: \&quot;102110\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-37.3159\&quot;,\n        \&quot;lng\&quot;: \&quot;81.1496\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;+6287-90830-0192\&quot;,\n    \&quot;website\&quot;: \&quot;tian.org\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;PT. Sukses Makmur\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;PT. Sukses Makmur Jaya\&quot;,\n      \&quot;bs\&quot;: \&quot;PT. Sukses Terus\&quot;\n    }\n  }&quot;,
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
      <webElementGuid>3585bf3f-eb95-4fc5-a7db-b65136079b52</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
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

//verifikasi status code 201
WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)

//verifikasi data yang telah di input
WS.verifyElementPropertyValue(response, 'name', 'Putri Titian')
WS.verifyElementPropertyValue(response, 'username', 'putri_titian')
WS.verifyElementPropertyValue(response, 'email', 'putritian@gmail.com')
WS.verifyElementPropertyValue(response, 'address.street', 'Jakarta')
WS.verifyElementPropertyValue(response, 'address.suite', 'Tebet. 556')
WS.verifyElementPropertyValue(response, 'address.city', 'Jaksel')
WS.verifyElementPropertyValue(response, 'address.zipcode', '102110')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, 'phone', '+6287-90830-0192')
WS.verifyElementPropertyValue(response, 'website', 'tian.org')
WS.verifyElementPropertyValue(response, 'company.name', 'PT. Sukses Makmur')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'PT. Sukses Makmur Jaya')
WS.verifyElementPropertyValue(response, 'company.bs', 'PT. Sukses Terus')
WS.verifyElementPropertyValue(response, 'id', '11')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
